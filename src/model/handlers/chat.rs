//! POST /v1/chat/completions handler.
//! OpenAI-compatible chat completions endpoint.

use axum::Json;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::response::sse::{Event, Sse};
use std::convert::Infallible;
use tokio_stream::StreamExt;

use crate::model::error::ModelError;
use crate::model::service;
use crate::model::state::AppState;
use crate::model::types::request::ChatCompletionRequest;

/// Handle chat completions request.
pub async fn post(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<ChatCompletionRequest>,
) -> impl IntoResponse {
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();

    tracing::trace!(
        "Chat request: agent={}, model={}, messages={}, stream={:?}, tools={}",
        user_agent,
        request.model,
        request.messages.len(),
        request.stream,
        request.tools.as_ref().map(|t| t.len()).unwrap_or(0),
    );

    // Validate required fields
    if request.messages.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ModelError::invalid_request(
                "messages array must not be empty",
            )),
        )
            .into_response();
    }

    if request.stream == Some(true) {
        // Stream response
        match service::process_stream(&state.config, &request, user_agent).await {
            Ok(chunks) => {
                let stream = tokio_stream::iter(chunks).map(|chunk| -> Result<Event, Infallible> {
                    Ok(Event::default().json_data(chunk).unwrap())
                });
                Sse::new(stream).into_response()
            }
            Err(error) => error.into_response(),
        }
    } else {
        // Sync response
        match service::process(&state.config, &request, user_agent).await {
            Ok(response) => {
                tracing::trace!("Chat response: {} choices", response.choices.len());
                Json(response).into_response()
            }
            Err(error) => error.into_response(),
        }
    }
}
