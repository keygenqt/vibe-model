//! GET /v1/models handler.
//! Returns available models list.

use axum::Json;
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
}

/// Handle models list request.
pub async fn get() -> Json<ModelsResponse> {
    Json(ModelsResponse {
        object: "list".to_string(),
        data: vec![ModelInfo {
            id: "vibe-model".to_string(),
            object: "model".to_string(),
            created: 0,
            owned_by: "vibe".to_string(),
        }],
    })
}
