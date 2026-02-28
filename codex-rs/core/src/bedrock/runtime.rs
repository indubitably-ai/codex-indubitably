use std::sync::Arc;

use serde_json::Value;
use thiserror::Error;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct ConverseRequest {
    pub payload: Value,
}

impl ConverseRequest {
    pub fn new(payload: Value) -> Self {
        Self { payload }
    }
}

#[derive(Debug)]
pub struct ConverseStream {
    receiver: mpsc::Receiver<Result<StreamChunk, BedrockError>>,
}

impl ConverseStream {
    pub fn new(receiver: mpsc::Receiver<Result<StreamChunk, BedrockError>>) -> Self {
        Self { receiver }
    }

    pub fn into_inner(self) -> mpsc::Receiver<Result<StreamChunk, BedrockError>> {
        self.receiver
    }
}

#[derive(Debug, Clone)]
pub enum StreamChunk {
    MessageDelta(Value),
    ToolUse(Value),
    ToolResult(Value),
    Usage(Value),
    Done,
}

#[derive(Debug, Error, Clone)]
pub enum BedrockError {
    #[error("transport error: {0}")]
    Transport(Arc<anyhow::Error>),

    #[error("invalid response: {0}")]
    InvalidResponse(String),

    #[error("request throttled")]
    Throttled,

    #[error("operation cancelled")]
    Cancelled,
}

impl BedrockError {
    pub fn transport(err: impl Into<anyhow::Error>) -> Self {
        Self::Transport(Arc::new(err.into()))
    }
}
