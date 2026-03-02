use anyhow::Context;
use eventsource_stream::Eventsource;
use futures::StreamExt;
use reqwest::StatusCode;
use serde_json::Value;
use tokio::sync::mpsc;
use tracing::debug;

use crate::bedrock::runtime::BedrockError;
use crate::bedrock::runtime::ConverseRequest;
use crate::bedrock::runtime::ConverseStream;
use crate::bedrock::runtime::StreamChunk;

const STREAM_BUFFER_SIZE: usize = 32;

#[derive(Clone, Debug)]
pub struct BedrockProxyRuntime {
    base_url: String,
    query_params: Option<std::collections::HashMap<String, String>>,
    headers: reqwest::header::HeaderMap,
    client: reqwest::Client,
}

impl BedrockProxyRuntime {
    pub fn new(
        base_url: String,
        query_params: Option<std::collections::HashMap<String, String>>,
        headers: reqwest::header::HeaderMap,
        client: reqwest::Client,
    ) -> Self {
        Self {
            base_url,
            query_params,
            headers,
            client,
        }
    }

    pub async fn converse_stream(
        &self,
        request: ConverseRequest,
        bearer_token: Option<&str>,
    ) -> Result<ConverseStream, BedrockError> {
        let response = self.send_request(&request, true, bearer_token).await?;
        let (tx, rx) = mpsc::channel(STREAM_BUFFER_SIZE);

        tokio::spawn(async move {
            stream_bedrock_sse(response, tx).await;
        });

        Ok(ConverseStream::new(rx))
    }

    fn invoke_url(&self, stream: bool) -> String {
        let trimmed = self.base_url.trim_end_matches('/');
        let base = if let Some(without_v1) = trimmed.strip_suffix("/v1") {
            without_v1
        } else {
            trimmed
        };
        if stream {
            format!("{base}/cli/bedrock/invoke?stream=true")
        } else {
            format!("{base}/cli/bedrock/invoke")
        }
    }

    async fn send_request(
        &self,
        request: &ConverseRequest,
        stream: bool,
        bearer_token: Option<&str>,
    ) -> Result<reqwest::Response, BedrockError> {
        let mut req = self.client.post(self.invoke_url(stream));

        if let Some(token) = bearer_token {
            req = req.bearer_auth(token);
        }

        if !self.headers.is_empty() {
            req = req.headers(self.headers.clone());
        }

        if let Some(query_params) = self.query_params.as_ref() {
            req = req.query(query_params);
        }

        let response = req
            .json(&request.payload)
            .send()
            .await
            .map_err(map_reqwest_error)?;

        let status = response.status();
        if status.is_success() {
            Ok(response)
        } else {
            let body = response.text().await.unwrap_or_default();
            Err(map_http_error(status, &body))
        }
    }
}

async fn stream_bedrock_sse(
    response: reqwest::Response,
    tx: mpsc::Sender<Result<StreamChunk, BedrockError>>,
) {
    let stream = response.bytes_stream();
    let mut events = stream.eventsource();

    while let Some(next) = events.next().await {
        let event = match next {
            Ok(event) => event,
            Err(error) => {
                let _ = tx.send(Err(BedrockError::transport(error))).await;
                return;
            }
        };

        if event.event == "bedrock.done" || event.data.trim() == "[DONE]" {
            let _ = tx.send(Ok(StreamChunk::Done)).await;
            return;
        }

        if event.event == "bedrock.chunk" || event.event.is_empty() {
            let chunk = match parse_bedrock_chunk(&event.data) {
                Ok(Some(chunk)) => chunk,
                Ok(None) => continue,
                Err(err) => {
                    let _ = tx.send(Err(err)).await;
                    return;
                }
            };

            if tx.send(Ok(chunk)).await.is_err() {
                return;
            }
        } else {
            debug!("ignoring bedrock SSE event: {}", event.event);
        }
    }
}

fn parse_bedrock_chunk(raw: &str) -> Result<Option<StreamChunk>, BedrockError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let value: Value = serde_json::from_str(trimmed)
        .context("failed to parse bedrock SSE payload")
        .map_err(BedrockError::transport)?;

    let kind = value
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| BedrockError::InvalidResponse("bedrock chunk missing type".to_string()))?;

    let payload = value.get("payload").cloned().unwrap_or(Value::Null);

    let chunk = match kind {
        "message_delta" => StreamChunk::MessageDelta(payload),
        "tool_use" => StreamChunk::ToolUse(payload),
        "tool_result" => StreamChunk::ToolResult(payload),
        "usage" => StreamChunk::Usage(payload),
        "done" => StreamChunk::Done,
        "cancelled" => return Err(BedrockError::Cancelled),
        other => {
            return Err(BedrockError::InvalidResponse(format!(
                "unknown bedrock chunk type {other}"
            )));
        }
    };

    Ok(Some(chunk))
}

fn map_http_error(status: StatusCode, body: &str) -> BedrockError {
    if matches!(
        status,
        StatusCode::TOO_MANY_REQUESTS | StatusCode::SERVICE_UNAVAILABLE
    ) {
        return BedrockError::Throttled;
    }

    if status == StatusCode::UNAUTHORIZED
        || (status == StatusCode::FORBIDDEN && is_authentication_error(body))
    {
        return BedrockError::InvalidResponse(
            "indubitably authentication expired; run `codex login --indubitably`".to_string(),
        );
    }

    if status == StatusCode::PAYMENT_REQUIRED {
        return BedrockError::InvalidResponse(
            "insufficient indubitably credits; run `codex login status --indubitably`".to_string(),
        );
    }

    let detail = parse_error_detail(body);
    let message = if let Some(detail) = detail {
        format!("bedrock proxy returned status {status}: {detail}")
    } else {
        format!("bedrock proxy returned status {status}")
    };

    BedrockError::InvalidResponse(message)
}

fn parse_error_detail(body: &str) -> Option<String> {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return None;
    }

    let Ok(parsed) = serde_json::from_str::<Value>(trimmed) else {
        return Some(trimmed.to_string());
    };
    if let Some(error) = parsed.get("error") {
        if let Some(message) = error.as_str() {
            return Some(message.to_string());
        }
        if let Some(message) = error.get("message").and_then(Value::as_str) {
            return Some(message.to_string());
        }
    }
    if let Some(message) = parsed.get("message").and_then(Value::as_str) {
        return Some(message.to_string());
    }

    Some(trimmed.to_string())
}

fn is_authentication_error(body: &str) -> bool {
    let message = parse_error_detail(body)
        .map(|msg| msg.to_ascii_lowercase())
        .unwrap_or_default();
    !message.is_empty()
        && (message.contains("unauthorized")
            || message.contains("authentication")
            || message.contains("not authenticated")
            || message.contains("token"))
}

fn map_reqwest_error(error: reqwest::Error) -> BedrockError {
    BedrockError::transport(anyhow::anyhow!(error))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn forbidden_model_not_allowed_does_not_map_to_auth_expired() {
        let err = map_http_error(StatusCode::FORBIDDEN, r#"{"error":"Model not allowed"}"#);
        let BedrockError::InvalidResponse(message) = err else {
            panic!("expected invalid response");
        };
        assert_eq!(
            message,
            "bedrock proxy returned status 403 Forbidden: Model not allowed"
        );
    }

    #[test]
    fn unauthorized_maps_to_auth_expired() {
        let err = map_http_error(StatusCode::UNAUTHORIZED, r#"{"error":"Unauthorized"}"#);
        let BedrockError::InvalidResponse(message) = err else {
            panic!("expected invalid response");
        };
        assert_eq!(
            message,
            "indubitably authentication expired; run `codex login --indubitably`"
        );
    }
}
