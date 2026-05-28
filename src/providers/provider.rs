//! API provider trait.
//! Each provider implements its own HTTP router.

use std::sync::Arc;

use axum::Router;

use crate::pipeline::pipeline::Pipeline;

/// API provider — creates HTTP router for a specific API type.
pub trait Provider: Send + Sync {
    /// Provider type identifier.
    fn key(&self) -> ProviderType;
    /// Build axum router.
    fn router(&self, pipeline: Arc<Pipeline>) -> Router;
}

/// Supported API provider types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderType {
    /// OpenAI-compatible API.
    Openai,
}

impl ProviderType {
    /// Human-readable provider name.
    pub fn name(&self) -> &'static str {
        match self {
            ProviderType::Openai => "OpenAI",
        }
    }
}
