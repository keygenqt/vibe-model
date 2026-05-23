//! Chat completion response types.
//! OpenAI-compatible response structures.

use serde::Serialize;

use crate::model::types::request::Message;

/// Chat completion response.
#[derive(Debug, Serialize)]
pub struct ChatCompletionResponse {
    /// Unique response identifier.
    pub id: String,
    /// Object type.
    pub object: String,
    /// Unix timestamp of creation.
    pub created: u64,
    /// Model name.
    pub model: String,
    /// Completion choices.
    pub choices: Vec<Choice>,
    /// Token usage statistics.
    pub usage: Usage,
}

/// Completion choice.
#[derive(Debug, Serialize)]
pub struct Choice {
    /// Choice index.
    pub index: u32,
    /// Generated message.
    pub message: Message,
    /// Reason generation stopped.
    pub finish_reason: String,
}

/// Token usage statistics.
#[derive(Debug, Serialize)]
pub struct Usage {
    /// Prompt tokens.
    pub prompt_tokens: u32,
    /// Completion tokens.
    pub completion_tokens: u32,
    /// Total tokens.
    pub total_tokens: u32,
}

/// Stream chunk for chat completion.
#[derive(Debug, Serialize)]
pub struct ChatCompletionChunk {
    /// Unique response identifier.
    pub id: String,
    /// Object type.
    pub object: String,
    /// Unix timestamp of creation.
    pub created: u64,
    /// Model name.
    pub model: String,
    /// Completion choices (delta).
    pub choices: Vec<ChoiceDelta>,
}

/// Stream choice with delta.
#[derive(Debug, Serialize)]
pub struct ChoiceDelta {
    /// Choice index.
    pub index: u32,
    /// Message delta (partial content).
    pub delta: Delta,
    /// Reason generation stopped (null until final chunk).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
}

/// Delta content for stream chunk.
#[derive(Debug, Serialize)]
pub struct Delta {
    /// Message role (assistant).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Partial content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl Default for ChatCompletionResponse {
    fn default() -> Self {
        Self {
            id: String::new(),
            object: "chat.completion".into(),
            created: 0,
            model: "vibe-model".into(),
            choices: vec![],
            usage: Usage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
            },
        }
    }
}
