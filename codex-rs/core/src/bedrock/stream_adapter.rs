use std::collections::HashMap;
use std::collections::HashSet;

use codex_protocol::models::ContentItem;
use codex_protocol::models::FunctionCallOutputPayload;
use codex_protocol::models::LocalShellAction;
use codex_protocol::models::LocalShellExecAction;
use codex_protocol::models::LocalShellStatus;
use codex_protocol::models::ResponseItem;
use codex_protocol::models::WebSearchAction;
use serde_json::Value;
use tokio::sync::mpsc;
use tracing::debug;
use tracing::warn;

use crate::bedrock::runtime::ConverseStream;
use crate::bedrock::runtime::StreamChunk;
use crate::bedrock::usage::parse_usage;
use crate::client_common::ResponseEvent;
use crate::client_common::ResponseStream;
use crate::client_common::tools::ToolSpec;
use crate::error::CodexErr;
use crate::protocol::TokenUsage;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ToolKind {
    Function,
    Freeform,
    LocalShell,
    WebSearch,
    Unknown,
}

pub(crate) fn adapt_converse_stream(
    conversation_id: String,
    tools: Vec<ToolSpec>,
    stream: ConverseStream,
) -> ResponseStream {
    let (tx, rx_event) = mpsc::channel(32);
    let tool_index = index_tools(tools);

    tokio::spawn(async move {
        if tx.send(Ok(ResponseEvent::Created)).await.is_err() {
            return;
        }

        let mut state = AdapterState {
            active_message: None,
            next_message_index: 0,
            usage: None,
            tool_index,
            call_kinds: HashMap::new(),
            seen_tool_calls: HashSet::new(),
            seen_tool_results: HashSet::new(),
        };

        let mut receiver = stream.into_inner();
        while let Some(next) = receiver.recv().await {
            match next {
                Ok(chunk) => {
                    if !state.handle_chunk(&tx, chunk).await {
                        break;
                    }
                }
                Err(err) => {
                    let _ = tx.send(Err(CodexErr::Stream(err.to_string(), None))).await;
                    return;
                }
            }
        }

        state.finish(&tx, &conversation_id).await;
    });

    ResponseStream { rx_event }
}

fn index_tools(tools: Vec<ToolSpec>) -> HashMap<String, ToolKind> {
    let mut map = HashMap::new();

    for tool in tools {
        match tool {
            ToolSpec::Function(_) => {
                map.insert(tool.name().to_string(), ToolKind::Function);
            }
            ToolSpec::ToolSearch { execution, .. } => {
                map.insert(execution, ToolKind::Function);
                map.insert("tool_search".to_string(), ToolKind::Function);
            }
            ToolSpec::Freeform(_) => {
                map.insert(tool.name().to_string(), ToolKind::Freeform);
            }
            ToolSpec::LocalShell {} => {
                map.insert("local_shell".to_string(), ToolKind::LocalShell);
                map.insert("shell".to_string(), ToolKind::LocalShell);
            }
            ToolSpec::WebSearch { .. } => {
                map.insert("web_search".to_string(), ToolKind::WebSearch);
                map.insert("web-search".to_string(), ToolKind::WebSearch);
            }
            ToolSpec::ImageGeneration { .. } => {
                map.insert("image_generation".to_string(), ToolKind::Unknown);
            }
        }
    }

    map
}

struct AdapterState {
    active_message: Option<ActiveMessage>,
    next_message_index: usize,
    usage: Option<TokenUsage>,
    tool_index: HashMap<String, ToolKind>,
    call_kinds: HashMap<String, ToolKind>,
    seen_tool_calls: HashSet<String>,
    seen_tool_results: HashSet<String>,
}

struct ActiveMessage {
    id: String,
    text: String,
}

impl AdapterState {
    async fn handle_chunk(
        &mut self,
        tx: &mpsc::Sender<crate::error::Result<ResponseEvent>>,
        chunk: StreamChunk,
    ) -> bool {
        match chunk {
            StreamChunk::MessageDelta(value) => self.handle_message_delta(tx, &value).await,
            StreamChunk::ToolUse(value) => self.handle_tool_use(tx, &value).await,
            StreamChunk::ToolResult(value) => self.handle_tool_result(tx, &value).await,
            StreamChunk::Usage(value) => {
                self.usage = usage_from_value(&value);
                true
            }
            StreamChunk::Done => false,
        }
    }

    async fn handle_message_delta(
        &mut self,
        tx: &mpsc::Sender<crate::error::Result<ResponseEvent>>,
        value: &Value,
    ) -> bool {
        let Some(delta) = extract_text_delta(value) else {
            debug!("bedrock message delta missing text: {value:?}");
            return true;
        };

        if !self.ensure_assistant_message_started(tx).await {
            return false;
        }
        if let Some(message) = self.active_message.as_mut() {
            message.text.push_str(delta);
        }

        tx.send(Ok(ResponseEvent::OutputTextDelta(delta.to_string())))
            .await
            .is_ok()
    }

    async fn handle_tool_use(
        &mut self,
        tx: &mpsc::Sender<crate::error::Result<ResponseEvent>>,
        value: &Value,
    ) -> bool {
        let Some(tool_use) = value.get("toolUse").and_then(Value::as_object) else {
            debug!("tool_use chunk missing payload: {value:?}");
            return true;
        };

        let Some(name) = tool_use.get("name").and_then(Value::as_str) else {
            debug!("tool_use missing name: {tool_use:?}");
            return true;
        };
        let Some(call_id) = tool_use.get("id").and_then(Value::as_str) else {
            debug!("tool_use missing id: {tool_use:?}");
            return true;
        };
        let input = tool_use.get("input").cloned().unwrap_or(Value::Null);

        if self.seen_tool_calls.contains(call_id) {
            return true;
        }
        if !self.flush_active_assistant_message(tx).await {
            return false;
        }
        self.seen_tool_calls.insert(call_id.to_string());

        let kind = self
            .tool_index
            .get(name)
            .copied()
            .unwrap_or(ToolKind::Unknown);
        self.call_kinds.insert(call_id.to_string(), kind);

        let item = match kind {
            ToolKind::Function => Some(function_call_item(name, call_id, &input)),
            ToolKind::Freeform | ToolKind::Unknown => {
                Some(custom_tool_call_item(name, call_id, &input))
            }
            ToolKind::LocalShell => match local_shell_call_item(call_id, &input) {
                Some(item) => Some(item),
                None => {
                    warn!("failed to parse local shell call input: {input:?}");
                    None
                }
            },
            ToolKind::WebSearch => Some(web_search_call_item(call_id, &input)),
        };

        if let Some(item) = item {
            tx.send(Ok(ResponseEvent::OutputItemDone(item)))
                .await
                .is_ok()
        } else {
            true
        }
    }

    async fn handle_tool_result(
        &mut self,
        tx: &mpsc::Sender<crate::error::Result<ResponseEvent>>,
        value: &Value,
    ) -> bool {
        let Some(tool_result) = value.get("toolResult").and_then(Value::as_object) else {
            debug!("tool_result chunk missing payload: {value:?}");
            return true;
        };
        let Some(tool_use_id) = tool_result.get("toolUseId").and_then(Value::as_str) else {
            debug!("tool_result missing tool_use_id: {tool_result:?}");
            return true;
        };

        if self.seen_tool_results.contains(tool_use_id) {
            return true;
        }
        if !self.flush_active_assistant_message(tx).await {
            return false;
        }
        self.seen_tool_results.insert(tool_use_id.to_string());

        let content = tool_result
            .get("content")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let flattened = flatten_tool_result_content(&content);

        let kind = self
            .call_kinds
            .get(tool_use_id)
            .copied()
            .unwrap_or(ToolKind::Unknown);

        let item = match kind {
            ToolKind::Function | ToolKind::LocalShell => Some(ResponseItem::FunctionCallOutput {
                call_id: tool_use_id.to_string(),
                output: FunctionCallOutputPayload::from_text(flattened),
            }),
            ToolKind::Freeform | ToolKind::WebSearch | ToolKind::Unknown => {
                Some(ResponseItem::CustomToolCallOutput {
                    call_id: tool_use_id.to_string(),
                    output: FunctionCallOutputPayload::from_text(flattened),
                })
            }
        };

        if let Some(item) = item {
            tx.send(Ok(ResponseEvent::OutputItemDone(item)))
                .await
                .is_ok()
        } else {
            true
        }
    }

    async fn finish(
        mut self,
        tx: &mpsc::Sender<crate::error::Result<ResponseEvent>>,
        conversation_id: &str,
    ) {
        if !self.flush_active_assistant_message(tx).await {
            return;
        }

        let completed = ResponseEvent::Completed {
            response_id: conversation_id.to_string(),
            token_usage: self.usage,
        };
        let _ = tx.send(Ok(completed)).await;
    }

    async fn ensure_assistant_message_started(
        &mut self,
        tx: &mpsc::Sender<crate::error::Result<ResponseEvent>>,
    ) -> bool {
        if self.active_message.is_some() {
            return true;
        }

        let id = format!("bedrock-msg-{}", self.next_message_index);
        self.next_message_index += 1;
        let started = ResponseItem::Message {
            id: Some(id.clone()),
            role: "assistant".to_string(),
            content: vec![ContentItem::OutputText {
                text: String::new(),
            }],
            end_turn: None,
            phase: None,
        };

        if tx
            .send(Ok(ResponseEvent::OutputItemAdded(started)))
            .await
            .is_err()
        {
            return false;
        }

        self.active_message = Some(ActiveMessage {
            id,
            text: String::new(),
        });
        true
    }

    async fn flush_active_assistant_message(
        &mut self,
        tx: &mpsc::Sender<crate::error::Result<ResponseEvent>>,
    ) -> bool {
        let Some(message) = self.active_message.take() else {
            return true;
        };
        if message.text.is_empty() {
            return true;
        }

        tx.send(Ok(ResponseEvent::OutputItemDone(ResponseItem::Message {
            id: Some(message.id),
            role: "assistant".to_string(),
            content: vec![ContentItem::OutputText { text: message.text }],
            end_turn: None,
            phase: None,
        })))
        .await
        .is_ok()
    }
}

fn extract_text_delta(value: &Value) -> Option<&str> {
    value
        .get("delta")
        .and_then(|delta| delta.get("text"))
        .and_then(Value::as_str)
        .or_else(|| value.get("text").and_then(Value::as_str))
}

fn usage_from_value(value: &Value) -> Option<TokenUsage> {
    parse_usage(value)
}

fn function_call_item(name: &str, call_id: &str, input: &Value) -> ResponseItem {
    ResponseItem::FunctionCall {
        id: None,
        name: name.to_string(),
        namespace: None,
        arguments: to_compact_json(input),
        call_id: call_id.to_string(),
    }
}

fn custom_tool_call_item(name: &str, call_id: &str, input: &Value) -> ResponseItem {
    ResponseItem::CustomToolCall {
        id: None,
        status: None,
        call_id: call_id.to_string(),
        name: name.to_string(),
        input: to_compact_json(input),
    }
}

fn local_shell_call_item(call_id: &str, input: &Value) -> Option<ResponseItem> {
    let command_value = input.get("command")?;
    let command = parse_command(command_value)?;
    let timeout_ms = input.get("timeout_ms").and_then(Value::as_u64);
    let working_directory = input
        .get("working_directory")
        .and_then(Value::as_str)
        .map(ToString::to_string);
    let env = parse_env(input.get("env"));
    let user = input
        .get("user")
        .and_then(Value::as_str)
        .map(ToString::to_string);

    Some(ResponseItem::LocalShellCall {
        id: None,
        call_id: Some(call_id.to_string()),
        status: LocalShellStatus::InProgress,
        action: LocalShellAction::Exec(LocalShellExecAction {
            command,
            timeout_ms,
            working_directory,
            env,
            user,
        }),
    })
}

fn web_search_call_item(call_id: &str, input: &Value) -> ResponseItem {
    let action = match input
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or_default()
    {
        "search" => WebSearchAction::Search {
            query: input
                .get("query")
                .and_then(Value::as_str)
                .map(str::to_string),
            queries: None,
        },
        _ => WebSearchAction::Other,
    };

    ResponseItem::WebSearchCall {
        id: Some(call_id.to_string()),
        status: Some("in_progress".to_string()),
        action: Some(action),
    }
}

fn parse_command(value: &Value) -> Option<Vec<String>> {
    match value {
        Value::String(s) => Some(vec![s.to_string()]),
        Value::Array(arr) => {
            let mut out = Vec::with_capacity(arr.len());
            for item in arr {
                out.push(item.as_str()?.to_string());
            }
            Some(out)
        }
        _ => None,
    }
}

fn parse_env(value: Option<&Value>) -> Option<HashMap<String, String>> {
    let Value::Object(map) = value? else {
        return None;
    };

    let mut out = HashMap::new();
    for (key, val) in map {
        if let Some(s) = val.as_str() {
            out.insert(key.clone(), s.to_string());
        }
    }

    if out.is_empty() { None } else { Some(out) }
}

fn to_compact_json(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| value.to_string())
}

fn flatten_tool_result_content(content: &[Value]) -> String {
    let mut out = String::new();
    for entry in content {
        if let Some(text) = entry.get("text").and_then(Value::as_str) {
            if !out.is_empty() {
                out.push('\n');
            }
            out.push_str(text);
        } else {
            if !out.is_empty() {
                out.push('\n');
            }
            out.push_str(&entry.to_string());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    fn stream_with_chunks(
        chunks: Vec<Result<StreamChunk, crate::bedrock::runtime::BedrockError>>,
    ) -> ConverseStream {
        let buffer = chunks.len().max(1);
        let (tx, rx) = mpsc::channel(buffer);
        for chunk in chunks {
            tx.try_send(chunk).expect("send chunk");
        }
        ConverseStream::new(rx)
    }

    #[tokio::test]
    async fn emits_output_item_added_before_text_delta() {
        let stream = stream_with_chunks(vec![
            Ok(StreamChunk::MessageDelta(json!({"delta":{"text":"Hello"}}))),
            Ok(StreamChunk::Done),
        ]);

        let events: Vec<_> = adapt_converse_stream("conv-1".to_string(), vec![], stream)
            .collect()
            .await;

        assert_eq!(events.len(), 5);
        assert!(matches!(events[0], Ok(ResponseEvent::Created)));
        assert!(matches!(
            &events[1],
            Ok(ResponseEvent::OutputItemAdded(ResponseItem::Message { .. }))
        ));
        match events[2].as_ref().expect("delta event") {
            ResponseEvent::OutputTextDelta(delta) => assert_eq!(delta, "Hello"),
            other => panic!("unexpected delta event: {other:?}"),
        }
        match &events[3] {
            Ok(ResponseEvent::OutputItemDone(ResponseItem::Message { content, .. })) => {
                assert_eq!(
                    content,
                    &vec![ContentItem::OutputText {
                        text: "Hello".to_string(),
                    }]
                );
            }
            other => panic!("unexpected item done event: {other:?}"),
        }
        assert!(matches!(events[4], Ok(ResponseEvent::Completed { .. })));
    }

    #[tokio::test]
    async fn flushes_text_message_before_tool_output() {
        let stream = stream_with_chunks(vec![
            Ok(StreamChunk::MessageDelta(json!({"delta":{"text":"Hello"}}))),
            Ok(StreamChunk::ToolUse(json!({
                "toolUse":{
                    "name":"shell",
                    "id":"call-1",
                    "input":{"command":"pwd"}
                }
            }))),
            Ok(StreamChunk::Done),
        ]);

        let events: Vec<_> =
            adapt_converse_stream("conv-2".to_string(), vec![ToolSpec::LocalShell {}], stream)
                .collect()
                .await;

        assert!(matches!(events[0], Ok(ResponseEvent::Created)));
        assert!(matches!(
            events[1],
            Ok(ResponseEvent::OutputItemAdded(ResponseItem::Message { .. }))
        ));
        match events[2].as_ref().expect("delta event") {
            ResponseEvent::OutputTextDelta(delta) => assert_eq!(delta, "Hello"),
            other => panic!("unexpected delta event: {other:?}"),
        }
        assert!(matches!(
            events[3],
            Ok(ResponseEvent::OutputItemDone(ResponseItem::Message { .. }))
        ));
        assert!(matches!(
            events[4],
            Ok(ResponseEvent::OutputItemDone(
                ResponseItem::LocalShellCall { .. }
            ))
        ));
        assert!(matches!(
            &events[5],
            Ok(ResponseEvent::Completed {
                response_id,
                token_usage: None
            }) if response_id == "conv-2"
        ));
    }
}
