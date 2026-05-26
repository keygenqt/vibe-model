//! GET /v1/models handler.
//! Returns available models list.

use std::sync::Arc;

use axum::Json;
use axum::extract::State;

use crate::pipeline::pipeline::Pipeline;

/// Handle models list request.
pub async fn get(State(pipeline): State<Arc<Pipeline>>) -> Json<serde_json::Value> {
    let identity = &pipeline.config().identity;
    Json(serde_json::json!({
        "object": "list",
        "data": [{
            "id": identity.id,
            "object": "model",
            "created": 0,
            "owned_by": "vibe"
        }]
    }))
}
