//! Chat completion request types.
//! OpenAI-compatible request structures.

use serde::Deserialize;
use serde::Serialize;

/// Chat completion request.
#[derive(Debug, Deserialize, Clone)]
pub struct ChatCompletionRequest {
    /// Model name (returned in response).
    pub model: String,
    /// Conversation messages.
    pub messages: Vec<Message>,
    /// Available tools for function calling.
    #[serde(default)]
    pub tools: Option<Vec<Tool>>,
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

/// Conversation message.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    /// Message role (system, user, assistant, tool).
    pub role: String,
    /// Message content (string or array of content parts).
    pub content: Option<Content>,
    /// Tool calls from assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    /// Tool call result identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

/// Message content — either a plain string or an array of content parts.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Content {
    /// Plain text content.
    Text(String),
    /// Array of content parts (text, images, etc.).
    Parts(Vec<ContentPart>),
}

/// A content part within a message (text or image).
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentPart {
    /// Part type: "text", "image_url", etc.
    #[serde(rename = "type")]
    pub part_type: String,
    /// Text content (for text parts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Image URL (for image parts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<ImageUrl>,
}

/// Image URL for vision content parts.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ImageUrl {
    /// Image URL or base64-encoded data.
    pub url: String,
}

/// Available tool definition.
#[derive(Debug, Deserialize, Clone)]
pub struct Tool {
    /// Function definition.
    pub function: FunctionDef,
}

/// Function definition for a tool.
#[derive(Debug, Deserialize, Clone)]
pub struct FunctionDef {
    /// Function name.
    pub name: String,
    /// Function description.
    pub description: Option<String>,
    /// Function parameters (JSON Schema).
    pub parameters: Option<serde_json::Value>,
}

/// Tool call from assistant.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ToolCall {
    /// Tool call identifier.
    pub id: String,
    /// Function call details.
    pub function: FunctionCall,
}

/// Function call arguments.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FunctionCall {
    /// Function name to call.
    pub name: String,
    /// Function arguments as JSON string.
    pub arguments: String,
}
