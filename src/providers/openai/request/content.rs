//! OpenAI message content types.

use serde::Deserialize;
use serde::Serialize;

use crate::providers::openai::request::tool::ToolCallRequest;

/// Message role.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

/// Conversation message.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MessageRequest {
    /// Message role (system, user, assistant, tool).
    pub role: Role,
    /// Message content (string or array of content parts).
    pub content: Option<ContentRequest>,
    /// Tool calls from assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallRequest>>,
    /// Tool call result identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

/// Message content — either a plain string or an array of content parts.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum ContentRequest {
    /// Plain text content.
    Text(String),
    /// Array of content parts (text, images, etc.).
    Parts(Vec<ContentPartRequest>),
}

/// A content part within a message (text or image).
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentPartRequest {
    /// Part type: "text", "image_url", etc.
    #[serde(rename = "type")]
    pub part_type: String,
    /// Text content (for text parts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Image URL (for image parts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<ImageUrlRequest>,
}

/// Image URL for vision content parts.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ImageUrlRequest {
    /// Image URL or base64-encoded data.
    pub url: String,
}

impl ContentRequest {
    pub fn as_text(&self) -> Option<String> {
        match self {
            ContentRequest::Text(text) => Some(text.clone()),
            ContentRequest::Parts(parts) => {
                let text = parts
                    .iter()
                    .filter_map(|p| p.text.clone())
                    .collect::<Vec<_>>()
                    .join("\n");
                if text.is_empty() { None } else { Some(text) }
            }
        }
    }
}
