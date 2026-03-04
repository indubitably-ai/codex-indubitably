use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_json::json;

use crate::client_common::tools::ToolSpec;
use crate::content_items_to_text;
use codex_protocol::models::ContentItem;
use codex_protocol::models::FunctionCallOutputPayload;
use codex_protocol::models::ResponseItem;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BedrockMessagesRequest {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    pub messages: Vec<BedrockMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<BedrockTool>>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct BedrockMessage {
    pub role: String,
    pub content: Vec<BedrockContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BedrockContent {
    Text {
        text: String,
    },
    #[serde(rename_all = "camelCase")]
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },
    #[serde(rename_all = "camelCase")]
    ToolResult {
        #[serde(rename = "tool_use_id")]
        tool_use_id: String,
        content: Vec<Value>,
    },
}

impl Default for BedrockContent {
    fn default() -> Self {
        Self::Text {
            text: String::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BedrockTool {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<Value>,
}

pub(crate) fn build_request(
    model: &str,
    system: Option<String>,
    history: &[ResponseItem],
    tools: &[ToolSpec],
    output_schema: Option<&Value>,
) -> BedrockMessagesRequest {
    let mut messages = Vec::new();
    for item in history {
        if let Some(message) = convert_item(item) {
            messages.push(message);
        }
    }

    let mut converted_tools = crate::bedrock::tools::convert_tools(tools);
    if let Some(schema) = output_schema {
        let schema_tool = BedrockTool {
            name: "codex_output_schema".to_string(),
            description: None,
            input_schema: Some(schema.clone()),
        };
        match &mut converted_tools {
            Some(tools) => tools.push(schema_tool),
            None => converted_tools = Some(vec![schema_tool]),
        }
    }

    BedrockMessagesRequest {
        model: model.to_string(),
        system: system.filter(|text| !text.trim().is_empty()),
        messages,
        tools: converted_tools,
    }
}

fn convert_item(item: &ResponseItem) -> Option<BedrockMessage> {
    match item {
        ResponseItem::Message { role, content, .. } => convert_message(role, content),
        ResponseItem::FunctionCall {
            name,
            call_id,
            arguments,
            ..
        } => Some(tool_use_message(
            call_id,
            name,
            parse_json(arguments).unwrap_or_else(|| Value::String(arguments.clone())),
        )),
        ResponseItem::CustomToolCall {
            call_id,
            name,
            input,
            ..
        } => Some(tool_use_message(
            call_id,
            name,
            parse_json(input).unwrap_or_else(|| Value::String(input.clone())),
        )),
        ResponseItem::LocalShellCall {
            call_id,
            id,
            action,
            ..
        } => Some(tool_use_message(
            call_id
                .as_deref()
                .or(id.as_deref())
                .unwrap_or("local_shell"),
            "shell",
            serde_json::to_value(action).unwrap_or_else(|_| json!({})),
        )),
        ResponseItem::FunctionCallOutput { call_id, output } => {
            let content = payload_to_text(output);
            Some(tool_result_message(call_id, content.as_str()))
        }
        ResponseItem::CustomToolCallOutput { call_id, output } => {
            let content = payload_to_text(output);
            Some(tool_result_message(call_id, content.as_str()))
        }
        ResponseItem::WebSearchCall { id, action, .. } => Some(tool_use_message(
            id.as_deref().unwrap_or("web_search"),
            "web_search",
            serde_json::to_value(action).unwrap_or_else(|_| json!({})),
        )),
        ResponseItem::Reasoning { .. }
        | ResponseItem::GhostSnapshot { .. }
        | ResponseItem::Compaction { .. }
        | ResponseItem::ImageGenerationCall { .. }
        | ResponseItem::Other => None,
    }
}

fn convert_message(role: &str, content: &[ContentItem]) -> Option<BedrockMessage> {
    let text = content_items_to_text(content);
    text.filter(|value| !value.trim().is_empty())
        .map(|text| BedrockMessage {
            role: normalize_role(role),
            content: vec![BedrockContent::Text { text }],
        })
}

fn normalize_role(role: &str) -> String {
    match role {
        "assistant" => "assistant".to_string(),
        "user" | "tool" => "user".to_string(),
        _ => "user".to_string(),
    }
}

fn tool_use_message(id: &str, name: &str, input: Value) -> BedrockMessage {
    BedrockMessage {
        role: "assistant".to_string(),
        content: vec![BedrockContent::ToolUse {
            id: if id.is_empty() {
                format!("{name}_call")
            } else {
                id.to_string()
            },
            name: name.to_string(),
            input,
        }],
    }
}

fn tool_result_message(tool_use_id: &str, output: &str) -> BedrockMessage {
    BedrockMessage {
        role: "user".to_string(),
        content: vec![BedrockContent::ToolResult {
            tool_use_id: tool_use_id.to_string(),
            content: vec![text_block(output)],
        }],
    }
}

fn payload_to_text(payload: &FunctionCallOutputPayload) -> String {
    payload.body.to_text().unwrap_or_default()
}

fn parse_json(input: &str) -> Option<Value> {
    serde_json::from_str::<Value>(input).ok()
}

fn text_block(text: &str) -> Value {
    let sanitized = if text.trim().is_empty() {
        "(no output)"
    } else {
        text
    };
    json!({ "type": "text", "text": sanitized })
}
