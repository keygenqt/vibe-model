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

use crate::pipeline::actions::complexity::ComplexityAction;
use crate::pipeline::context::PipelineContext;
use crate::pipeline::pipeline::Pipeline;
use crate::providers::agents::AgentType;
use crate::providers::openai::request::chat::ChatCompletionRequest;
use crate::providers::openai::response::error::ErrorResponse;

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

    let mut ctx = match PipelineContext::new(agent, pipeline.config().clone(), prompt) {
        Ok(ctx) => ctx,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal(&e.to_string())),
            )
                .into_response();
        }
    };

    if let Err(e) = pipeline.execute(Box::new(ComplexityAction), &mut ctx).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::internal(&e.to_string())),
        )
            .into_response();
    }

    if request.stream == Some(true) {
        // Stream response
        let chunks = ctx.stream_chunks.unwrap_or_default();
        let stream = tokio_stream::iter(chunks).map(|chunk| -> Result<Event, Infallible> {
            Ok(Event::default().json_data(chunk).unwrap())
        });
        Sse::new(stream).into_response()
    } else {
        // Sync response
        let response = ctx.response.unwrap_or_default();
        Json(serde_json::json!({
            "object": "chat.completion",
            "model": request.model,
            "choices": [{
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": response
                },
                "finish_reason": "stop"
            }],
            "usage": {
                "prompt_tokens": 0,
                "completion_tokens": 0,
                "total_tokens": 0
            }
        }))
        .into_response()
    }
}
