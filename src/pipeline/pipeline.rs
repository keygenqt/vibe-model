//! Pipeline executor.
//! Orchestrates the execution of pipeline actions.

use anyhow::Result;

use crate::configs::app::AppConfig;
use crate::pipeline::action::PipelineAction;
use crate::pipeline::actions::ast::AstAction;
use crate::pipeline::actions::cache::CacheAction;
use crate::pipeline::actions::cluster::ClusterAction;
use crate::pipeline::actions::complexity::ComplexityAction;
use crate::pipeline::actions::context::ContextAction;
use crate::pipeline::actions::enrich::EnrichAction;
use crate::pipeline::actions::format::FormatAction;
use crate::pipeline::actions::log::LogAction;
use crate::pipeline::actions::parser::ParseAction;
use crate::pipeline::actions::species::SpeciesAction;
use crate::pipeline::actions::system::SystemAction;
use crate::pipeline::context::PipelineContext;
use crate::providers::provider::ProviderType;

/// Pipeline — orchestrates the execution of actions.
pub struct Pipeline {
    config: AppConfig,
    provider: ProviderType,
}

impl Pipeline {
    /// Create pipeline from config.
    pub fn new(config: AppConfig, provider: ProviderType) -> Self {
        Self { config, provider }
    }

    /// Get pipeline config.
    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    /// Get provider type.
    pub fn provider(&self) -> &ProviderType {
        &self.provider
    }

    /// Default pipeline actions in execution order.
    fn list(&self) -> Vec<Box<dyn PipelineAction>> {
        vec![
            // Return cached response for known prompts
            Box::new(CacheAction),
            // Parse raw prompt into structured model
            Box::new(ParseAction),
            // Determine prompt species/type
            Box::new(SpeciesAction),
            // Handle system queries without model
            Box::new(SystemAction),
            // Evaluate prompt complexity
            Box::new(ComplexityAction),
            // Enrich prompt with AST from vibe-ast
            Box::new(AstAction),
            // Add conversation history and environment context
            Box::new(ContextAction),
            // Add dynamic context (time, system info)
            Box::new(EnrichAction),
            // Send prompt to LLM cluster
            Box::new(ClusterAction),
            // Format response for specific agent
            Box::new(FormatAction),
            // Log pipeline state for debugging
            Box::new(LogAction),
        ]
    }

    /// Execute pipeline with starting action.
    pub async fn execute(&self, ctx: &mut PipelineContext) -> Result<()> {
        for action in self.list() {
            if action.enabled(ctx) {
                tracing::trace!("Start: {}", action.name());
                action.execute(ctx).await?;
            }
        }
        Ok(())
    }
}
