use serde_json::json;
use serde_json::to_value;

use crate::bedrock::messages_adapter::BedrockTool;
use crate::client_common::tools::FreeformTool;
use crate::client_common::tools::ToolSpec;

pub(crate) fn convert_tools(tools: &[ToolSpec]) -> Option<Vec<BedrockTool>> {
    let mut out = Vec::new();

    for tool in tools {
        match tool {
            ToolSpec::Function(function) => {
                let schema = to_value(&function.parameters).unwrap_or_else(|_| json!({}));
                let description = (!function.description.trim().is_empty())
                    .then_some(function.description.clone());
                out.push(BedrockTool {
                    name: function.name.clone(),
                    description,
                    input_schema: Some(schema),
                });
            }
            ToolSpec::Freeform(freeform) => {
                let description = (!freeform.description.trim().is_empty())
                    .then_some(freeform.description.clone());
                out.push(BedrockTool {
                    name: freeform.name.clone(),
                    description,
                    input_schema: Some(freeform_schema(freeform)),
                });
            }
            ToolSpec::LocalShell {} => out.push(BedrockTool {
                name: "local_shell".to_string(),
                description: Some("Execute commands on the local shell.".to_string()),
                input_schema: Some(json!({
                    "type": "object",
                    "properties": {
                        "command": { "type": "array", "items": { "type": "string" } },
                        "timeout_ms": { "type": "integer" },
                        "working_directory": { "type": "string" }
                    },
                    "required": ["command"]
                })),
            }),
            ToolSpec::WebSearch { .. } => out.push(BedrockTool {
                name: "web_search".to_string(),
                description: Some("Perform an internet search.".to_string()),
                input_schema: Some(json!({
                    "type": "object",
                    "properties": {
                        "query": { "type": "string" }
                    },
                    "required": ["query"]
                })),
            }),
            ToolSpec::ImageGeneration {} => out.push(BedrockTool {
                name: "image_generation".to_string(),
                description: Some("Generate an image.".to_string()),
                input_schema: Some(json!({
                    "type": "object",
                    "properties": {},
                })),
            }),
        }
    }

    if out.is_empty() { None } else { Some(out) }
}

fn freeform_schema(tool: &FreeformTool) -> serde_json::Value {
    json!({
        "type": tool.format.r#type,
        "syntax": tool.format.syntax,
        "definition": tool.format.definition,
    })
}
