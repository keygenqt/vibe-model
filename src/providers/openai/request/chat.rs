//! OpenAI chat completion request types.

use serde::Deserialize;

use crate::providers::openai::request::{
    content::{ContentRequest, MessageRequest},
    tool::ToolRequest,
};

/// Chat completion request.
#[derive(Debug, Deserialize, Clone)]
pub struct ChatCompletionRequest {
    /// Model name (returned in response).
    pub model: String,
    /// Conversation messages.
    pub messages: Vec<MessageRequest>,
    /// Available tools for function calling.
    #[serde(default)]
    pub tools: Option<Vec<ToolRequest>>,
    /// Tool choice mode.
    #[serde(default)]
    pub tool_choice: Option<String>,
    /// Whether to stream the response.
    #[serde(default)]
    pub stream: Option<bool>,
    /// Sampling temperature.
    #[serde(default)]
    pub temperature: Option<f32>,
    /// Maximum tokens to generate.
    #[serde(default)]
    pub max_tokens: Option<u32>,
}

impl ChatCompletionRequest {
    /// Extract prompt from last user message.
    pub fn to_prompt(&self) -> String {
        self.messages
            .iter()
            .filter(|m| m.role == "user")
            .last()
            .and_then(|m| m.content.as_ref())
            .map(|c| match c {
                ContentRequest::Text(text) => text.clone(),
                ContentRequest::Parts(parts) => parts
                    .iter()
                    .filter_map(|p| p.text.clone())
                    .collect::<Vec<_>>()
                    .join("\n"),
            })
            .unwrap_or_default()
    }
}
