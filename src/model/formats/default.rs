//! Default response format.
//! Returns plain text message.

use super::ResponseFormat;
use crate::model::types::request::{Content, Message};

/// Default format handler (plain text).
pub struct DefaultFormat;

impl ResponseFormat for DefaultFormat {
    fn format_message(&self, content: &str) -> Message {
        Message {
            role: "assistant".into(),
            content: Some(Content::Text(content.into())),
            tool_calls: None,
            tool_call_id: None,
        }
    }
}
