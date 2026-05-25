//! OpenAI model list response types.

use serde::Serialize;

/// Model list response.
#[derive(Debug, Serialize)]
pub struct ModelsResponse {
    /// Object type.
    pub object: String,
    /// Available models.
    pub data: Vec<ModelInfo>,
}

/// Model information.
#[derive(Debug, Serialize)]
pub struct ModelInfo {
    /// Model identifier.
    pub id: String,
    /// Object type.
    pub object: String,
    /// Unix timestamp of creation.
    pub created: u64,
    /// Organization that owns the model.
    pub owned_by: String,
    /// Maximum context window in tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_window: Option<u32>,
    /// Maximum output tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Model capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<String>>,
}
