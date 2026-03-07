use codex_core::ModelProviderInfo;
use codex_core::built_in_model_providers;
use codex_protocol::openai_models::ModelInfo;
use serde_json::Value;
use serde_json::json;

use crate::bedrock_fixtures::load_converse_payload;

const BEDROCK_PROVIDER_ID: &str = "bedrock";
const TEST_MODEL_SLUG: &str = "claude-3-7-sonnet";

pub fn provider_for_mock_server_uri(server_uri: &str) -> ModelProviderInfo {
    let mut provider = built_in_model_providers()
        .get(BEDROCK_PROVIDER_ID)
        .unwrap_or_else(|| panic!("bedrock provider should exist"))
        .clone();
    provider.base_url = Some(format!("{server_uri}/v1"));
    provider.experimental_bearer_token = Some("bedrock-test-token".to_string());
    provider
}

pub fn test_model_info() -> ModelInfo {
    let description = fixture_description();
    serde_json::from_value(json!({
        "slug": TEST_MODEL_SLUG,
        "display_name": TEST_MODEL_SLUG,
        "description": description,
        "default_reasoning_level": "medium",
        "supported_reasoning_levels": [
            {"effort": "medium", "description": "medium"}
        ],
        "shell_type": "shell_command",
        "visibility": "list",
        "supported_in_api": true,
        "priority": 1,
        "upgrade": null,
        "base_instructions": "base instructions",
        "model_messages": null,
        "supports_reasoning_summaries": false,
        "support_verbosity": false,
        "default_verbosity": null,
        "apply_patch_tool_type": null,
        "truncation_policy": {"mode": "bytes", "limit": 10000},
        "supports_parallel_tool_calls": false,
        "context_window": 272000,
        "auto_compact_token_limit": null,
        "experimental_supported_tools": []
    }))
    .unwrap_or_else(|err| panic!("deserialize test model info: {err}"))
}

fn fixture_description() -> String {
    let payload = load_converse_payload("converse_basic.json");
    payload
        .pointer("/output/message/content/0/text")
        .and_then(Value::as_str)
        .unwrap_or("bedrock test model")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn provider_for_mock_server_uri_uses_bedrock_base_url() {
        let provider = provider_for_mock_server_uri("http://localhost:8181");
        assert_eq!(
            provider.base_url,
            Some("http://localhost:8181/v1".to_string())
        );
        assert_eq!(
            provider.experimental_bearer_token.as_deref(),
            Some("bedrock-test-token")
        );
    }

    #[test]
    fn test_model_info_uses_fixture_backed_description() {
        let info = test_model_info();
        assert_eq!(info.slug, TEST_MODEL_SLUG);
        assert_eq!(info.description, Some("Hello from Bedrock!".to_string()));
    }
}
