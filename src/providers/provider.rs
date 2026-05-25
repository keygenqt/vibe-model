//! API provider trait.
//! Each provider implements its own HTTP router.

use std::sync::Arc;

use axum::Router;

use crate::pipeline::pipeline::Pipeline;

/// API provider — creates HTTP router for a specific API type.
pub trait Provider: Send + Sync {
    /// Build axum router with pipeline.
    fn router(&self, pipeline: Arc<Pipeline>) -> Router;
    /// Provider name for logging and config.
    fn name(&self) -> &'static str;
}
