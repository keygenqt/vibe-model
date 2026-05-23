//! Server configuration.
//! Defines HTTP server bind address and port.

use serde::Deserialize;
use serde::Serialize;

/// HTTP server settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server bind address (127.0.0.1 for local only, 0.0.0.0 for all interfaces).
    pub host: String,
    /// Server port (default: 9333).
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 9333,
        }
    }
}
