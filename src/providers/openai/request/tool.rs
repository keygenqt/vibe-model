//! OpenAI tool calling types.

use serde::Deserialize;
use serde::Serialize;

/// Available tool definition.
#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct ToolRequest {
    /// Function definition.
    pub function: FunctionDefRequest,
}

/// Function definition for a tool.
#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct FunctionDefRequest {
    /// Function name.
    pub name: String,
    /// Function description.
    pub description: Option<String>,
    /// Function parameters (JSON Schema).
    pub parameters: Option<serde_json::Value>,
}

/// Tool call from assistant.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ToolCallRequest {
    /// Tool call identifier.
    pub id: String,
    /// Function call details.
    pub function: FunctionCallRequest,
}

/// Function call arguments.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FunctionCallRequest {
    /// Function name to call.
    pub name: String,
    /// Function arguments as JSON string.
    pub arguments: String,
}
