//! Pipeline execution context.
//! Passed through all pipeline actions.

use anyhow::Result;
use vibe_cluster::{Cluster, ConnectionParams, Provider};

use crate::configs::app::AppConfig;
use crate::pipeline::actions::cache::CacheActionContext;
use crate::pipeline::actions::cluster::ClusterActionContext;
use crate::pipeline::actions::complexity::ComplexityActionContext;
use crate::pipeline::actions::format::FormatActionContext;
use crate::pipeline::actions::parser::ParseActionContext;
use crate::pipeline::actions::species::SpeciesActionContext;
use crate::pipeline::actions::system::SystemActionContext;
use crate::providers::agents::AgentType;
use crate::providers::openai::request::chat::ChatCompletionRequest;
use crate::providers::provider::ProviderType;

/// Pipeline execution context.
#[derive(Debug, Clone)]
pub struct PipelineContext {
    /// Application config.
    pub config: AppConfig,
    /// Detected AI agent type.
    pub agent: AgentType,
    /// API provider type.
    pub provider: ProviderType,
    /// Full chat completion request.
    pub request: ChatCompletionRequest,
    /// Cache action context.
    pub cache_action: Option<CacheActionContext>,
    /// Cluster action context.
    pub cluster_action: Option<ClusterActionContext>,
    /// Complexity action context.
    pub complexity_action: Option<ComplexityActionContext>,
    /// Format action context.
    pub format_action: Option<FormatActionContext>,
    /// Parse action context.
    pub parse_action: Option<ParseActionContext>,
    /// Species action context.
    pub species_action: Option<SpeciesActionContext>,
    /// System action context.
    pub system_action: Option<SystemActionContext>,
}

impl PipelineContext {
    /// Create new pipeline context.
    pub fn new(
        config: AppConfig,
        agent: AgentType,
        provider: ProviderType,
        request: &ChatCompletionRequest,
    ) -> Result<Self> {
        Ok(Self {
            config,
            agent,
            provider,
            request: request.clone(),
            cache_action: None,
            cluster_action: None,
            complexity_action: None,
            format_action: None,
            parse_action: None,
            species_action: None,
            system_action: None,
        })
    }

    /// Get response from any action that produced one (cache, system, or cluster).
    pub fn get_response(&self) -> Result<String> {
        if let Some(cache_ctx) = &self.cache_action {
            return Ok(cache_ctx.response.clone());
        }
        if let Some(system_ctx) = &self.system_action {
            return Ok(system_ctx.response.clone());
        }
        if let Some(cluster_ctx) = &self.cluster_action {
            return Ok(cluster_ctx.response.clone());
        }
        Err(anyhow::anyhow!("No response available"))
    }

    /// Create vibe-cluster from config.
    pub fn create_cluster(&self) -> Result<Cluster> {
        let connections: Vec<ConnectionParams> = self
            .config
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
