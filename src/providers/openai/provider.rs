//! OpenAI API provider.
//! Implements Provider trait with OpenAI-compatible endpoints.

use std::sync::Arc;

use axum::Router;
use axum::routing::get;
use axum::routing::post;

use crate::pipeline::pipeline::Pipeline;
use crate::providers::openai::route;
use crate::providers::provider::Provider;
use crate::providers::provider::ProviderType;

/// OpenAI API provider.
pub struct OpenAiProvider;

impl Provider for OpenAiProvider {
    /// Provider type identifier.
    fn key(&self) -> ProviderType {
        ProviderType::Openai
    }

    /// Build axum router.
    fn router(&self, pipeline: Arc<Pipeline>) -> Router {
        Router::new()
            .route("/v1/chat/completions", post(route::chat::post))
            .route("/v1/models", get(route::models::get))
            .with_state(pipeline)
    }
}
