//! POST /v1/chat/completions handler.
//! OpenAI-compatible chat completions endpoint.

use std::convert::Infallible;
use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::response::sse::{Event, Sse};
use tokio_stream::StreamExt;

use crate::pipeline::context::PipelineContext;
use crate::pipeline::pipeline::Pipeline;
use crate::providers::agents::AgentType;
use crate::providers::openai::error::ErrorResponse;
use crate::providers::openai::request::chat::ChatCompletionRequest;

/// Handle chat completions request.
pub async fn post(
    State(pipeline): State<Arc<Pipeline>>,
    headers: HeaderMap,
    Json(request): Json<ChatCompletionRequest>,
) -> impl IntoResponse {
    let agent = AgentType::from_user_agent(
        headers
            .get("user-agent")
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default(),
    );

    if request.messages.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::invalid_request(
                "messages array must not be empty",
            )),
        )
            .into_response();
    }

    let prompt = request.to_prompt();

    tracing::trace!(
        "Chat request: agent={:?}, model={}, messages={}, stream={:?}, tools={}",
        agent,
        request.model,
        request.messages.len(),
        request.stream,
        request.tools.as_ref().map(|t| t.len()).unwrap_or(0),
    );

    let mut ctx = match PipelineContext::new(
        pipeline.config().clone(),
        agent,
        pipeline.provider().clone(),
        prompt,
    ) {
        Ok(ctx) => ctx,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal(&e.to_string())),
            )
                .into_response();
        }
    };

    if let Err(e) = pipeline.execute(&mut ctx).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::internal(&e.to_string())),
        )
            .into_response();
    }

    if request.stream == Some(true) {
        // Stream response — formatter already returns SSE string
        let sse_data = ctx
            .format_action
            .as_ref()
            .and_then(|f| f.stream_chunks.clone())
            .unwrap_or_default();

        // Parse JSON array of chunks and send as SSE
        let chunks: Vec<serde_json::Value> = serde_json::from_str(&sse_data).unwrap_or_default();
        let stream = tokio_stream::iter(chunks).map(|chunk| -> Result<Event, Infallible> {
            Ok(Event::default().json_data(chunk).unwrap())
        });
        Sse::new(stream).into_response()
    } else {
        // Sync response — formatter already returns JSON string
        let response = ctx
            .format_action
            .as_ref()
            .and_then(|f| f.response.clone())
            .unwrap_or_default();

        // Parse formatter JSON and return as response
        let json: serde_json::Value = serde_json::from_str(&response).unwrap_or_default();
        Json(json).into_response()
    }
}
