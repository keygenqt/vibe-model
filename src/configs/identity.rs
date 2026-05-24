//! Model information configuration.
//! Defines how vibe-model presents itself to clients.

use serde::Deserialize;
use serde::Serialize;

/// Configuration for model presentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityConfig {
    /// Model identifier shown to clients.
    pub id: String,
    /// Maximum context window in tokens.
    pub context_window: u32,
    /// Maximum output tokens.
    pub max_tokens: u32,
    /// Model capabilities.
    pub capabilities: Vec<String>,
}

impl Default for IdentityConfig {
    /// Creates default model configuration.
    fn default() -> Self {
        Self {
            id: "vibe-model".to_string(),
            context_window: 200000,
            max_tokens: 32000,
            capabilities: vec![
                "code".to_string(),
                "reasoning".to_string(),
                "tools".to_string(),
                "multilingual".to_string(),
            ],
        }
    }
}
