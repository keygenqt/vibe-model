//! Model information configuration.
//! Defines how vibe-model presents itself to clients.

use serde::Deserialize;
use serde::Serialize;

/// Configuration for model presentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityConfig {
    /// Model identifier shown to clients.
    pub id: String,
}

impl Default for IdentityConfig {
    /// Creates default model configuration.
    fn default() -> Self {
        Self {
            id: "vibe-model".to_string(),
        }
    }
}
