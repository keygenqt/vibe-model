//! Complexity estimator configuration.
//! Separate from cluster — one estimator for the whole pipeline.

use serde::Deserialize;
use serde::Serialize;

/// Configuration for the complexity estimator model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimatorConfig {
    /// Provider type: "ollama", "deepseek", "qwen".
    pub provider: String,
    /// API endpoint URL.
    pub host: String,
    /// Model name to use for complexity estimation.
    pub model: String,
    /// Number of parallel connections to this provider (default 1).
    pub parallel: usize,
}

impl Default for EstimatorConfig {
    /// Creates default estimator configuration.
    fn default() -> Self {
        Self {
            provider: "ollama".to_string(),
            host: "http://localhost:11434".to_string(),
            model: "qwen2.5-coder:14b-instruct".to_string(),
            parallel: 1,
        }
    }
}
