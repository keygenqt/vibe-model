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

    /// Create vibe-cluster from config filtered by complexity score.
    pub fn create_cluster(&self, complexity: f32) -> Result<Cluster> {
        let comp_step = (complexity * 100.0).round() as usize;
        let connections: Vec<ConnectionParams> = self
            .config
            .cluster
            .iter()
            .filter(|c| {
                let from_step = (c.complexity_from * 100.0).round() as usize;
                let to_step = (c.complexity_to * 100.0).round() as usize;
                comp_step >= from_step && comp_step <= to_step
            })
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

        if connections.is_empty() {
            anyhow::bail!("No nodes found for complexity: {:.2}", complexity);
        }

        Cluster::new(connections).map_err(|e| anyhow::anyhow!(e.to_string()))
    }

    /// Create estimator from config.
    pub fn create_estimator(&self) -> Result<Cluster> {
        let provider = match self.config.estimator.provider.as_str() {
            "ollama" => Provider::Ollama,
            "deepseek" => Provider::DeepSeek,
            "qwen" => Provider::Qwen,
            other => anyhow::bail!("Unknown provider: {}", other),
        };

        let connection = ConnectionParams {
            provider,
            host: self.config.estimator.host.clone(),
            model: self.config.estimator.model.clone(),
            temperature: Some(0.0), // hardcoded for estimator
            seed: Some(42),
            num_ctx: Some(2048),
            num_predict: Some(10),
            timeout_secs: Some(30),
            api_key: None,
            parallel: self.config.estimator.parallel,
        };

        Cluster::new(vec![connection]).map_err(|e| anyhow::anyhow!(e.to_string()))
    }
}
