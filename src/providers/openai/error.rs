//! Error response type for OpenAI provider.

use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use serde::Serialize;

/// Error response.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

/// Error detail.
#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    /// Human-readable error message.
    pub message: String,
    /// Error type identifier.
    #[serde(rename = "type")]
    pub error_type: String,
}

impl ErrorResponse {
    /// Create an internal error.
    pub fn internal(message: &str) -> Self {
        Self {
            error: ErrorDetail {
                message: message.to_string(),
                error_type: "internal_error".into(),
            },
        }
    }

    /// Create an invalid request error.
    pub fn invalid_request(message: &str) -> Self {
        Self {
            error: ErrorDetail {
                message: message.to_string(),
                error_type: "invalid_request_error".into(),
            },
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}
