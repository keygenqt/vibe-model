//! Pipeline executor.
//! Orchestrates the execution of pipeline actions.

use anyhow::Result;

use crate::configs::app::AppConfig;
use crate::pipeline::action::PipelineAction;
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

    /// Execute pipeline with starting action.
    pub async fn execute(
        &self,
        start: Box<dyn PipelineAction>,
        ctx: &mut PipelineContext,
    ) -> Result<()> {
        let mut queue: Vec<Box<dyn PipelineAction>> = vec![start];
        while let Some(action) = queue.pop() {
            tracing::trace!(
                "Executing: {}, prompt_len={}, complexity={:?}, species={:?}",
                action.name(),
                ctx.prompt.len(),
                ctx.complexity,
                ctx.species,
            );
            let next = action.execute(ctx).await?;
            tracing::trace!(
                "Done: {}, next_actions={}, response_len={}",
                action.name(),
                next.len(),
                ctx.response.as_ref().map(|r| r.len()).unwrap_or(0),
            );
            queue.extend(next);
        }
        Ok(())
    }
}
