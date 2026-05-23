//! Application state shared across request handlers.

use std::sync::Arc;

use axum::extract::FromRef;

use crate::configs::app::AppConfig;

/// Application state shared across request handlers.
#[derive(Clone, FromRef)]
pub struct AppState {
    /// Application configuration.
    pub config: Arc<AppConfig>,
    // @todo vibe_cluster::Cluster
}
