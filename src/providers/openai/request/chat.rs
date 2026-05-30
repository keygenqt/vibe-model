//! OpenAI chat completion request types.

use serde::Deserialize;

use crate::providers::openai::request::{
    content::{MessageRequest, Role},
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
    /// Whether to stream the response.
    #[serde(default)]
    pub stream: Option<bool>,
}

#[allow(dead_code)]
impl ChatCompletionRequest {
    /// Get system prompt.
    pub fn system_prompt(&self) -> Option<String> {
        self.messages
            .iter()
            .find(|m| m.role == Role::System)
            .and_then(|m| m.content.as_ref().and_then(|c| c.as_text()))
    }

    /// Get conversation history (all messages except last user).
    pub fn conversation_history(&self) -> Vec<(Role, String)> {
        self.messages
            .iter()
            .filter_map(|m| {
                Some((
                    m.role.clone(),
                    m.content.as_ref().and_then(|c| c.as_text())?,
                ))
            })
            .collect()
    }

    /// Get last user message.
    pub fn user_prompt(&self) -> Option<String> {
        self.messages
            .iter()
            .filter(|m| m.role == Role::User)
            .last()
            .and_then(|m| m.content.as_ref().and_then(|c| c.as_text()))
    }

    /// Get tool call results.
    pub fn tool_results(&self) -> Vec<(String, String)> {
        self.messages
            .iter()
            .filter(|m| m.role == Role::Tool)
            .filter_map(|m| {
                let content = m.content.as_ref().and_then(|c| c.as_text())?;
                let tool_call_id = m.tool_call_id.clone()?;
                Some((tool_call_id, content))
            })
            .collect()
    }

    /// Get available tools.
    pub fn available_tools(&self) -> Vec<String> {
        self.tools
            .iter()
            .flat_map(|t| t.iter().map(|tool| tool.function.name.clone()))
            .collect()
    }
}
