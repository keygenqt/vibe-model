//! Application configuration management for vibe-model.
//! Handles loading, saving, and server configuration.

use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use crate::configs::cluster::ClusterConfig;
use crate::configs::estimator::EstimatorConfig;
use crate::configs::identity::IdentityConfig;
use crate::configs::log_level::LogLevel;
use crate::configs::server::ServerConfig;
use crate::exit_error;
use crate::utils;
use crate::utils::constants;
use crate::utils::json5::Json5;
use crate::utils::path;

/// Global config path override.
static GLOBAL_CONFIG_PATH: OnceLock<Option<PathBuf>> = OnceLock::new();

/// Main application configuration structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Configuration version.
    pub version: String,
    /// Log level for terminal/tracing output.
    pub log_level: LogLevel,
    /// HTTP server settings.
    pub server: ServerConfig,
    /// Model presentation settings for /v1/models endpoint.
    pub identity: IdentityConfig,
    /// Complexity estimator configuration.
    pub estimator: EstimatorConfig,
    /// LLM cluster nodes (local and cloud models).
    pub cluster: Vec<ClusterConfig>,
}

impl Default for AppConfig {
    /// Creates default configuration with safe defaults.
    fn default() -> Self {
        Self {
            version: constants::CONFIG_VERSION.to_string(),
            log_level: LogLevel::default(),
            server: ServerConfig::default(),
            identity: IdentityConfig::default(),
            estimator: EstimatorConfig::default(),
            cluster: vec![ClusterConfig::default()],
        }
    }
}

impl AppConfig {
    /// Sets the global config path override.
    pub fn set_global_config(path: Option<PathBuf>) {
        if let Some(ref p) = path {
            if !p.exists() {
                exit_error!("Config file not found: {}", p.display());
            }
            if p.extension().map_or(true, |ext| ext != "json5") {
                exit_error!("Config file must be .json5: {}", p.display());
            }
        }
        if GLOBAL_CONFIG_PATH.get().is_none() {
            let _ = GLOBAL_CONFIG_PATH.set(path);
        }
    }

    /// Returns the config file path.
    pub fn config_path() -> PathBuf {
        Self::config_dir().join(constants::CONFIG_FILE_NAME)
    }

    /// Returns the config directory.
    pub fn config_dir() -> PathBuf {
        if let Some(Some(config_path)) = GLOBAL_CONFIG_PATH.get() {
            config_path
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| path::config_dir())
        } else {
            path::config_dir()
        }
    }

    /// Load config from file.
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path();

        let config = if !config_path.exists() {
            let config = AppConfig::default();
            config.save()?;
            config
        } else {
            let content = fs::read_to_string(&config_path)?;
            let config: AppConfig = json5::from_str(&content).map_err(|e| {
                anyhow::anyhow!(
                    "Failed to parse config file.\nDelete {} and run again to create a new one.\n{}",
                    config_path.display(),
                    e
                )
            })?;

            if config.version != constants::CONFIG_VERSION {
                return Err(anyhow::anyhow!(
                    "Config version mismatch: found {}, expected {}\nDelete {} and run again.",
                    config.version,
                    constants::CONFIG_VERSION,
                    config_path.display()
                ));
            }

            config
        };

        Self::init_tracing(&config.log_level);
        Ok(config)
    }

    fn init_tracing(level: &LogLevel) {
        crate::utils::macros::set_log_level(level);
        if crate::utils::macros::is_tracing() {
            let filter = format!("vibe_model={}", level.as_str());
            let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
        }
    }

    /// Save config to file in JSON5 format.
    pub fn save(&self) -> Result<()> {
        let config_dir = Self::config_dir();
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }
        let commits = vec![
            Json5::Commit("version", vec!["Configuration version (do not modify)"]),
            Json5::Commit(
                "log_level",
                vec!["Log level: cli, error, warn, info, debug, trace"],
            ),
            Json5::Commit(
                "server",
                vec![
                    "HTTP server configuration",
                    "host - Bind address (127.0.0.1 for local only, 0.0.0.0 for all interfaces)",
                    "port - Server port (default: 3000)",
                ],
            ),
            Json5::Commit(
                "identity",
                vec![
                    "Model presentation settings for /v1/models endpoint",
                    "",
                    "id - Model identifier shown to clients",
                ],
            ),
            Json5::Commit(
                "estimator",
                vec![
                    "Complexity estimator configuration.",
                    "  CRITICAL ROLE: This model judges prompt difficulty (0.0-1.0) to route",
                    "  requests to the right model. A smarter estimator gives more accurate",
                    "  complexity scores, so use the most capable model you can afford.",
                    "  Results are cached via semantic search — the estimator runs only when",
                    "  a genuinely new prompt pattern is detected.",
                    "",
                    "provider - Provider type: ollama, deepseek, qwen",
                    "host - API endpoint",
                    "model - LLM model name",
                    "parallel - Number of parallel connections (default 1)",
                ],
            ),
            Json5::Commit(
                "cluster",
                vec![
                    "LLM cluster nodes (local and cloud models).",
                    "",
                    "provider - Provider type: ollama, deepseek, qwen",
                    "host - API endpoint",
                    "model - LLM model name",
                    "timeout_secs - Request timeout in seconds",
                    "complexity_from - Lower bound of complexity range (0.0-1.0)",
                    "complexity_to - Upper bound of complexity range (0.0-1.0)",
                    "temperature - Sampling temperature (0.0 - 1.0)",
                    "seed - Random seed for reproducibility",
                    "num_ctx - Context window size",
                    "num_predict - Maximum tokens to generate",
                    "api_key - API key for cloud providers",
                    "parallel - Number of parallel connections (default 1)",
                ],
            ),
        ];
        let json = json5::to_string(self)?;
        let content = utils::json5::add_commits(json, commits);
        fs::write(Self::config_path(), content)?;
        Ok(())
    }

    /// Validate configuration and return errors for misconfigurations.
    pub fn validate() -> Result<()> {
        let config = AppConfig::load()?;

        // Check at least one cluster node exists.
        if config.cluster.is_empty() {
            return Err(anyhow::anyhow!(
                "Cluster is empty. Add at least one LLM node to the cluster array."
            ));
        }

        // Check complexity ranges cover 0-100 (representing 0.00-1.00).
        let mut covered = [false; 101];
        for node in &config.cluster {
            let from = (node.complexity_from * 100.0).round() as usize;
            let to = (node.complexity_to * 100.0).round() as usize;
            let start = from.min(100);
            let end = to.min(100);
            for step in start..=end {
                covered[step] = true;
            }
        }
        if let Some(gap_index) = covered.iter().position(|&step_covered| !step_covered) {
            return Err(anyhow::anyhow!(
                "Complexity gap detected: step {:.2} is not covered by any model.",
                gap_index as f32 / 100.0
            ));
        }

        Ok(())
    }
}
