use crate::protocol::TokenUsage;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct UsagePayload {
    #[serde(default, alias = "inputTokens")]
    pub input_tokens: Option<i64>,
    #[serde(default, alias = "cachedInputTokens")]
    pub cached_input_tokens: Option<i64>,
    #[serde(
        default,
        alias = "cacheReadInputTokens",
        alias = "cache_read_input_tokens"
    )]
    pub cache_read_input_tokens: Option<i64>,
    #[serde(
        default,
        alias = "cacheCreationInputTokens",
        alias = "cache_creation_input_tokens"
    )]
    pub cache_creation_input_tokens: Option<i64>,
    #[serde(default, alias = "outputTokens")]
    pub output_tokens: Option<i64>,
    #[serde(default, alias = "reasoningOutputTokens")]
    pub reasoning_output_tokens: Option<i64>,
    #[serde(default, alias = "totalTokens")]
    pub total_tokens: Option<i64>,
}

impl From<UsagePayload> for TokenUsage {
    fn from(payload: UsagePayload) -> Self {
        let uncached_input_tokens = payload.input_tokens.unwrap_or_default();
        let cache_read_input_tokens = payload
            .cache_read_input_tokens
            .or(payload.cached_input_tokens)
            .unwrap_or_default();
        let cache_creation_input_tokens = payload.cache_creation_input_tokens.unwrap_or_default();
        let output_tokens = payload.output_tokens.unwrap_or_default();
        let reasoning_output_tokens = payload.reasoning_output_tokens.unwrap_or_default();

        let has_bedrock_cache_fields = payload.cache_read_input_tokens.is_some()
            || payload.cache_creation_input_tokens.is_some();
        let input_tokens = if has_bedrock_cache_fields {
            uncached_input_tokens + cache_read_input_tokens + cache_creation_input_tokens
        } else {
            uncached_input_tokens
        };

        let total_tokens = payload
            .total_tokens
            .unwrap_or(input_tokens + output_tokens + reasoning_output_tokens);

        TokenUsage {
            input_tokens,
            cached_input_tokens: cache_read_input_tokens,
            output_tokens,
            reasoning_output_tokens,
            total_tokens,
        }
    }
}

pub fn parse_usage(value: &serde_json::Value) -> Option<TokenUsage> {
    let payload = serde_json::from_value::<UsagePayload>(value.clone()).ok()?;
    let has_any = payload.input_tokens.is_some()
        || payload.cached_input_tokens.is_some()
        || payload.output_tokens.is_some()
        || payload.reasoning_output_tokens.is_some()
        || payload.total_tokens.is_some();
    has_any.then_some(payload.into())
}
