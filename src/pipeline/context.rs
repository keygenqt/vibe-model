//! Pipeline execution context.
//! Passed through all pipeline actions.

use anyhow::Result;
use vibe_cluster::{Cluster, ConnectionParams, Provider};

use crate::configs::app::AppConfig;
use crate::pipeline::actions::complexity::ComplexityType;
use crate::pipeline::actions::species::SpeciesType;
use crate::providers::agents::AgentType;
use crate::providers::openai::response::stream::ChatCompletionChunkResponse;

/// Pipeline execution context.
pub struct PipelineContext {
    /// Application config.
    pub config: AppConfig,
    /// Detected AI agent type.
    pub agent: AgentType,
    /// Current prompt text.
    pub prompt: String,
    /// Cluster for model calls.
    pub cluster: Cluster,
    /// Evaluated prompt complexity.
    pub complexity: Option<ComplexityType>,
    /// Detected prompt species/type.
    pub species: Option<SpeciesType>,
    /// Response text from model.
    pub response: Option<String>,
    /// Stream chunks for streaming responses.
    pub stream_chunks: Option<Vec<ChatCompletionChunkResponse>>,
}

impl PipelineContext {
    /// Create new pipeline context.
    pub fn new(agent: AgentType, config: AppConfig, prompt: String) -> Result<Self> {
        let cluster = Self::create_cluster(&config)?;
        Ok(Self {
            agent,
            config,
            cluster,
            prompt,
            response: None,
            complexity: None,
            species: None,
            stream_chunks: None,
        })
    }

    /// Create vibe-cluster from config.
    fn create_cluster(config: &AppConfig) -> Result<Cluster> {
        let connections: Vec<ConnectionParams> = config
            .cluster
            .iter()
            .map(|c| {
                let provider = match c.provider.as_str() {
                    "ollama" => Provider::Ollama,
                    "deepseek" => Provider::DeepSeek,
                    "qwen" => Provider::Qwen,
                    other => anyhow::bail!("Unknown provider: {}", other),
                };
                Ok(ConnectionParams {
                    provider,
                    host: c.host.clone(),
                    model: c.model.clone(),
                    temperature: Some(c.temperature),
                    seed: Some(c.seed),
                    num_ctx: Some(c.num_ctx),
                    num_predict: Some(c.num_predict),
                    timeout_secs: Some(c.timeout_secs),
                    api_key: c.api_key.clone(),
                    parallel: c.parallel,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Cluster::new(connections).map_err(|e| anyhow::anyhow!(e.to_string()))
    }
}
