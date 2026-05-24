//! Response format handlers for different AI clients.
//! Each client expects a specific response format.

use crate::model::types::request::Message;

pub mod cline;
pub mod default;
pub mod openai;

/// Known AI client identifiers in user-agent.
pub const AGENT_CLINE: &str = "Cline";
pub const AGENT_CONTINUE: &str = "Continue";
pub const AGENT_AIDER: &str = "Aider";

/// Trait for formatting model responses.
pub trait ResponseFormat: Send + Sync {
    /// Format plain text into client-specific message.
    fn format_message(&self, content: &str) -> Message;

    /// Clean up client-specific prompt before sending to model.
    fn clean_prompt(&self, prompt: &str) -> String {
        prompt.to_string()
    }
}

/// Detect client from user-agent and return appropriate format.
pub fn detect(user_agent: &str) -> Box<dyn ResponseFormat> {
    if user_agent.contains(AGENT_CLINE) {
        Box::new(cline::ClineFormat)
    } else if user_agent.contains(AGENT_CONTINUE) || user_agent.contains(AGENT_AIDER) {
        Box::new(openai::OpenAiFormat)
    } else {
        Box::new(default::DefaultFormat)
    }
}
