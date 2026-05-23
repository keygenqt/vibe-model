//! Error types for model responses.

use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use serde::Serialize;

/// Model error response.
#[derive(Debug, Serialize)]
pub struct ModelError {
    pub error: ModelErrorDetail,
}

/// Error detail with message and type.
#[derive(Debug, Serialize)]
pub struct ModelErrorDetail {
    /// Human-readable error message.
    pub message: String,
    /// Error type identifier.
    #[serde(rename = "type")]
    pub error_type: String,
    /// Optional error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl ModelError {
    /// Create an internal error.
    pub fn internal(message: &str) -> Self {
        Self {
            error: ModelErrorDetail {
                message: message.to_string(),
                error_type: "internal_error".into(),
                code: None,
            },
        }
    }

    /// Create an invalid request error.
    pub fn invalid_request(message: &str) -> Self {
        Self {
            error: ModelErrorDetail {
                message: message.to_string(),
                error_type: "invalid_request_error".into(),
                code: None,
            },
        }
    }
}

impl IntoResponse for ModelError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}
