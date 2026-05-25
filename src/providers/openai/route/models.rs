//! GET /v1/models handler.
//! Returns available models list.

use std::sync::Arc;

use axum::Json;
use axum::extract::State;

use crate::pipeline::pipeline::Pipeline;
use crate::providers::openai::response::models::{ModelInfo, ModelsResponse};

/// Handle models list request.
pub async fn get(State(pipeline): State<Arc<Pipeline>>) -> Json<ModelsResponse> {
    let identity = &pipeline.config().identity;
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
