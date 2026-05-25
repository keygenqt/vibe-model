//! OpenAI chat completion response types.

use serde::Serialize;

use crate::providers::openai::request::content::MessageRequest;

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
    pub choices: Vec<ChoiceResponse>,
    /// Token usage statistics.
    pub usage: UsageResponse,
}

/// Completion choice.
#[derive(Debug, Serialize)]
pub struct ChoiceResponse {
    /// Choice index.
    pub index: u32,
    /// Generated message.
    pub message: MessageRequest,
    /// Reason generation stopped.
    pub finish_reason: String,
}

/// Token usage statistics.
#[derive(Debug, Serialize, Clone)]
pub struct UsageResponse {
    /// Prompt tokens.
    pub prompt_tokens: u32,
    /// Completion tokens.
    pub completion_tokens: u32,
    /// Total tokens.
    pub total_tokens: u32,
}

impl Default for ChatCompletionResponse {
    fn default() -> Self {
        Self {
            id: String::new(),
            object: "chat.completion".into(),
            created: 0,
            model: "vibe-model".into(),
            choices: vec![],
            usage: UsageResponse {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
            },
        }
    }
}

impl ChatCompletionResponse {
    /// Create response from message.
    pub fn from_message(model: &str, message: MessageRequest) -> Self {
        let has_tool_calls = message.tool_calls.is_some();
        Self {
            id: format!("chatcmpl-{}", uuid::Uuid::new_v4()),
            object: "chat.completion".into(),
            created: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            model: model.into(),
            choices: vec![ChoiceResponse {
                index: 0,
                message,
                finish_reason: if has_tool_calls {
                    "tool_calls".into()
                } else {
                    "stop".into()
                },
            }],
            usage: UsageResponse {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
            },
        }
    }
}
