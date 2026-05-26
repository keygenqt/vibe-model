//! Pipeline executor.
//! Orchestrates the execution of pipeline actions.

use anyhow::Result;

use crate::configs::app::AppConfig;
use crate::pipeline::action::PipelineAction;
use crate::pipeline::actions::cache::CacheAction;
use crate::pipeline::actions::clean::CleanAction;
use crate::pipeline::actions::cluster::ClusterAction;
use crate::pipeline::actions::complexity::ComplexityAction;
use crate::pipeline::actions::format::FormatAction;
use crate::pipeline::actions::species::SpeciesAction;
use crate::pipeline::actions::system::SystemAction;
use crate::pipeline::context::PipelineContext;

/// Pipeline — orchestrates the execution of actions.
pub struct Pipeline {
    config: AppConfig,
}

impl Pipeline {
    /// Create pipeline from config.
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    /// Get pipeline config.
    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    /// Default pipeline actions in execution order.
    fn list(&self) -> Vec<Box<dyn PipelineAction>> {
        vec![
            // Return cached response for known prompts
            Box::new(CacheAction),
            // Determine prompt species/type
            Box::new(SpeciesAction),
            // Handle system queries without model
            Box::new(SystemAction),
            // Evaluate prompt complexity
            Box::new(ComplexityAction),
            // Clean agent-specific garbage from prompt
            Box::new(CleanAction),
            // Send prompt to LLM cluster
            Box::new(ClusterAction),
            // Format response for specific agent
            Box::new(FormatAction),
        ]
    }

    /// Execute pipeline with starting action.
    pub async fn execute(&self, ctx: &mut PipelineContext) -> Result<()> {
        tracing::debug!("Prompt: {}...", ctx.get_preview());
        for action in self.list() {
            if action.enabled(ctx) {
                tracing::debug!("Start: {}", action.name());
                action.execute(ctx).await?;
                tracing::trace!("Done, context: {:?}", ctx);
            }
        }
        Ok(())
    }
}
