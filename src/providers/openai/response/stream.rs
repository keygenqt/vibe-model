//! OpenAI stream response types.

use serde::Serialize;

use crate::providers::openai::{request::tool::ToolCallRequest, response::chat::UsageResponse};

/// Stream chunk for chat completion.
#[derive(Debug, Serialize, Clone)]
pub struct ChatCompletionChunkResponse {
    /// Unique response identifier.
    pub id: String,
    /// Object type (always "chat.completion.chunk").
    pub object: String,
    /// Unix timestamp of creation.
    pub created: u64,
    /// Model name.
    pub model: String,
    /// Completion choices (delta).
    pub choices: Vec<ChoiceDeltaResponse>,
    /// Token usage statistics (final chunk only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<UsageResponse>,
}

/// Stream choice with delta.
#[derive(Debug, Serialize, Clone)]
pub struct ChoiceDeltaResponse {
    /// Choice index.
    pub index: u32,
    /// Message delta (partial content).
    pub delta: DeltaResponse,
    /// Reason generation stopped (null until final chunk).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
}

/// Delta content for stream chunk.
#[derive(Debug, Serialize, Clone)]
pub struct DeltaResponse {
    /// Message role (assistant).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Partial content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Partial tool calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallRequest>>,
}

impl Default for ChatCompletionChunkResponse {
    fn default() -> Self {
        Self {
            id: String::new(),
            object: "chat.completion.chunk".into(),
            created: 0,
            model: String::new(),
            choices: vec![],
            usage: None,
        }
    }
}
