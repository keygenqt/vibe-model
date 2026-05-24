//! OpenAI JSON response format.
//! Returns standard OpenAI message format.

use super::ResponseFormat;
use crate::model::types::request::{Content, Message};

/// OpenAI format handler.
pub struct OpenAiFormat;

impl ResponseFormat for OpenAiFormat {
    fn format_message(&self, content: &str) -> Message {
        Message {
            role: "assistant".into(),
            content: Some(Content::Text(content.into())),
            tool_calls: None,
            tool_call_id: None,
        }
    }
}
