//! Chat completion response types.
//! OpenAI-compatible response structures.

use serde::Serialize;

use crate::model::types::request::{Message, ToolCall};

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
    /// Partial tool calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
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

impl ChatCompletionResponse {
    /// Create response from message.
    pub fn from_message(model: &str, message: Message) -> Self {
        let has_tool_calls = message.tool_calls.is_some();
        Self {
            id: format!("chatcmpl-{}", uuid::Uuid::new_v4()),
            object: "chat.completion".into(),
            created: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            model: model.into(),
            choices: vec![Choice {
                index: 0,
                message,
                finish_reason: if has_tool_calls {
                    "tool_calls".into()
                } else {
                    "stop".into()
                },
            }],
            usage: Usage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
            },
        }
    }
}
