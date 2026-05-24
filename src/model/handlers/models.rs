//! GET /v1/models handler.
//! Returns available models list.

use axum::{Json, extract::State};
use serde::Serialize;

use crate::model::state::AppState;

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

/// Handle models list request.
pub async fn get(State(state): State<AppState>) -> Json<ModelsResponse> {
    let identity = &state.config.identity;
    Json(ModelsResponse {
        object: "list".to_string(),
        data: vec![ModelInfo {
            id: identity.id.clone(),
            object: "model".to_string(),
            created: 0,
            owned_by: "vibe".to_string(),
            context_window: Some(identity.context_window),
            max_tokens: Some(identity.max_tokens),
            capabilities: Some(identity.capabilities.clone()),
        }],
    })
}
