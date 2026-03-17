use std::sync::Arc;

use async_trait::async_trait;
use codex_otel::SessionTelemetry;
use codex_protocol::ThreadId;
use codex_protocol::config_types::ReasoningSummary as ReasoningSummaryConfig;
use codex_protocol::openai_models::ModelInfo;
use codex_protocol::openai_models::ReasoningEffort as ReasoningEffortConfig;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderName;
use reqwest::header::HeaderValue;

use crate::AuthManager;
use crate::bedrock::messages_adapter;
use crate::bedrock::proxy_runtime::BedrockProxyRuntime;
use crate::bedrock::runtime::BedrockError;
use crate::bedrock::runtime::ConverseRequest;
use crate::bedrock::stream_adapter::adapt_converse_stream;
use crate::client_common::Prompt;
use crate::client_common::ResponseStream;
use crate::default_client::build_reqwest_client;
use crate::error::CodexErr;
use crate::error::Result;
use crate::indubitably_auth::load_access_token_for_base_url;
use crate::model_provider_info::BEDROCK_PROVIDER_ID;
use crate::model_provider_info::ModelProviderInfo;
use crate::util::command_with_args;

#[async_trait]
pub trait BedrockRuntimeAdapter: std::fmt::Debug + Send + Sync {
    #[allow(clippy::too_many_arguments)]
    async fn stream(
        &self,
        prompt: &Prompt,
        model_info: &ModelInfo,
        session_telemetry: &SessionTelemetry,
        effort: Option<ReasoningEffortConfig>,
        summary: ReasoningSummaryConfig,
        turn_metadata_header: Option<&str>,
    ) -> Result<ResponseStream>;
}

/// Default adapter used when Bedrock runtime wiring has not been configured.
#[derive(Debug, Default)]
pub struct UnconfiguredBedrockRuntimeAdapter;

#[async_trait]
impl BedrockRuntimeAdapter for UnconfiguredBedrockRuntimeAdapter {
    async fn stream(
        &self,
        _prompt: &Prompt,
        _model_info: &ModelInfo,
        _session_telemetry: &SessionTelemetry,
        _effort: Option<ReasoningEffortConfig>,
        _summary: ReasoningSummaryConfig,
        _turn_metadata_header: Option<&str>,
    ) -> Result<ResponseStream> {
        Err(CodexErr::UnsupportedOperation(
            "Bedrock runtime adapter is not configured".to_string(),
        ))
    }
}

#[derive(Debug)]
pub struct ProxyBedrockRuntimeAdapter {
    runtime: BedrockProxyRuntime,
    provider: ModelProviderInfo,
    auth_manager: Option<Arc<AuthManager>>,
    conversation_id: ThreadId,
}

impl ProxyBedrockRuntimeAdapter {
    pub fn new(
        provider: ModelProviderInfo,
        auth_manager: Option<Arc<AuthManager>>,
        conversation_id: ThreadId,
    ) -> Option<Self> {
        let base_url = provider.base_url.clone()?;
        if base_url.trim().is_empty() {
            return None;
        }

        let headers = provider_headers(&provider);
        let runtime = BedrockProxyRuntime::new(
            base_url,
            provider.query_params.clone(),
            headers,
            build_reqwest_client(),
        );

        Some(Self {
            runtime,
            provider,
            auth_manager,
            conversation_id,
        })
    }

    async fn resolve_bearer_token(&self) -> Result<Option<String>> {
        if let Some(api_key) = self.provider.api_key()? {
            return Ok(Some(api_key));
        }

        if let Some(token) = self.provider.experimental_bearer_token.clone() {
            return Ok(Some(token));
        }

        if let Some(base_url) = self.provider.base_url.as_deref()
            && let Some(token) = load_access_token_for_base_url(base_url)
        {
            return Ok(Some(token));
        }

        if let Some(manager) = self.auth_manager.as_ref()
            && let Some(auth) = manager.auth().await
        {
            return Ok(Some(auth.get_token()?));
        }

        Ok(None)
    }
}

#[async_trait]
impl BedrockRuntimeAdapter for ProxyBedrockRuntimeAdapter {
    async fn stream(
        &self,
        prompt: &Prompt,
        model_info: &ModelInfo,
        _session_telemetry: &SessionTelemetry,
        _effort: Option<ReasoningEffortConfig>,
        _summary: ReasoningSummaryConfig,
        _turn_metadata_header: Option<&str>,
    ) -> Result<ResponseStream> {
        let history = prompt.get_formatted_input();
        let system = (!prompt.base_instructions.text.trim().is_empty())
            .then_some(prompt.base_instructions.text.clone());

        let request_payload = messages_adapter::build_request(
            model_info.slug.as_str(),
            system,
            &history,
            &prompt.tools,
            prompt.output_schema.as_ref(),
        );

        let request = ConverseRequest::new(serde_json::to_value(request_payload)?);
        let Some(bearer_token) = self.resolve_bearer_token().await? else {
            return Err(CodexErr::Stream(
                format!(
                    "indubitably authentication expired; run `{}`",
                    command_with_args("login --indubitably")
                ),
                None,
            ));
        };

        let stream = self
            .runtime
            .converse_stream(request, Some(&bearer_token))
            .await
            .map_err(map_bedrock_error)?;

        Ok(adapt_converse_stream(
            self.conversation_id.to_string(),
            prompt.tools.clone(),
            stream,
        ))
    }
}

pub fn build_default_bedrock_runtime_adapter(
    provider_id: &str,
    provider: &ModelProviderInfo,
    auth_manager: Option<Arc<AuthManager>>,
    conversation_id: &ThreadId,
) -> Arc<dyn BedrockRuntimeAdapter> {
    if provider_id.eq_ignore_ascii_case(BEDROCK_PROVIDER_ID)
        && let Some(adapter) =
            ProxyBedrockRuntimeAdapter::new(provider.clone(), auth_manager, *conversation_id)
    {
        return Arc::new(adapter);
    }

    Arc::new(UnconfiguredBedrockRuntimeAdapter)
}

fn provider_headers(provider: &ModelProviderInfo) -> HeaderMap {
    let mut headers = HeaderMap::new();

    if let Some(extra) = &provider.http_headers {
        for (name, value) in extra {
            if let (Ok(parsed_name), Ok(parsed_value)) = (
                HeaderName::from_bytes(name.as_bytes()),
                HeaderValue::from_str(value),
            ) {
                headers.insert(parsed_name, parsed_value);
            }
        }
    }

    if let Some(env_headers) = &provider.env_http_headers {
        for (name, env_key) in env_headers {
            if let Ok(value) = std::env::var(env_key)
                && !value.trim().is_empty()
                && let (Ok(parsed_name), Ok(parsed_value)) = (
                    HeaderName::from_bytes(name.as_bytes()),
                    HeaderValue::from_str(&value),
                )
            {
                headers.insert(parsed_name, parsed_value);
            }
        }
    }

    headers
}

fn map_bedrock_error(err: BedrockError) -> CodexErr {
    match err {
        BedrockError::Transport(source) => CodexErr::Stream(source.to_string(), None),
        BedrockError::InvalidResponse(message) => CodexErr::Stream(message, None),
        BedrockError::Throttled => CodexErr::Stream("bedrock request throttled".to_string(), None),
        BedrockError::Cancelled => CodexErr::Stream("bedrock request cancelled".to_string(), None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model_provider_info::BEDROCK_PROVIDER_ID;
    use crate::model_provider_info::built_in_model_providers;
    use pretty_assertions::assert_eq;
    use serial_test::serial;

    struct EnvGuard {
        key: &'static str,
        previous: Option<String>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: impl Into<String>) -> Self {
            let previous = std::env::var(key).ok();
            // SAFETY: tests use this guard in a scoped manner and restore on drop.
            unsafe {
                std::env::set_var(key, value.into());
            }
            Self { key, previous }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            // SAFETY: tests use this guard in a scoped manner and restore on drop.
            unsafe {
                match self.previous.take() {
                    Some(value) => std::env::set_var(self.key, value),
                    None => std::env::remove_var(self.key),
                }
            }
        }
    }

    #[test]
    fn proxy_adapter_new_uses_default_proxy_base_url() {
        let provider = built_in_model_providers()
            .get(BEDROCK_PROVIDER_ID)
            .expect("bedrock provider should exist")
            .clone();
        let adapter = ProxyBedrockRuntimeAdapter::new(provider, None, ThreadId::new());
        assert!(adapter.is_some());
    }

    #[tokio::test]
    #[serial]
    async fn resolve_bearer_token_ignores_aws_credentials() {
        let mut provider = built_in_model_providers()
            .get(BEDROCK_PROVIDER_ID)
            .expect("bedrock provider should exist")
            .clone();
        provider.base_url = Some("https://bedrock-runtime.us-east-1.amazonaws.com".to_string());

        let adapter = ProxyBedrockRuntimeAdapter::new(provider, None, ThreadId::new())
            .expect("adapter should be constructed");

        let _aws_access_key_id = EnvGuard::set("AWS_ACCESS_KEY_ID", "AKIA_TEST_ACCESS_KEY");
        let _aws_secret_access_key =
            EnvGuard::set("AWS_SECRET_ACCESS_KEY", "test-secret-access-key");
        let _aws_session_token = EnvGuard::set("AWS_SESSION_TOKEN", "test-session-token");

        let token = adapter
            .resolve_bearer_token()
            .await
            .expect("token resolution should not fail");
        assert_eq!(token, None);
    }

    #[tokio::test]
    #[serial]
    async fn resolve_bearer_token_prefers_provider_env_key() {
        let mut provider = built_in_model_providers()
            .get(BEDROCK_PROVIDER_ID)
            .expect("bedrock provider should exist")
            .clone();
        provider.base_url = Some("https://api.indubitably.ai".to_string());
        provider.env_key = Some("BEDROCK_TEST_API_TOKEN".to_string());
        provider.experimental_bearer_token = Some("inline-token".to_string());

        let adapter = ProxyBedrockRuntimeAdapter::new(provider, None, ThreadId::new())
            .expect("adapter should be constructed");

        let _env_token = EnvGuard::set("BEDROCK_TEST_API_TOKEN", "env-token");

        let token = adapter
            .resolve_bearer_token()
            .await
            .expect("token resolution should not fail");
        assert_eq!(token, Some("env-token".to_string()));
    }
}
