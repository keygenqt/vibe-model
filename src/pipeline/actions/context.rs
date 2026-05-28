//! Context enrichment action.
//! Adds conversation history and environment context to the prompt.

use anyhow::Result;

use crate::pipeline::action::PipelineAction;
use crate::pipeline::context::PipelineContext;

/// Enriches prompt with context (history, environment, files).
pub struct ContextAction;

#[async_trait::async_trait]
impl PipelineAction for ContextAction {
    fn name(&self) -> &'static str {
        "Context"
    }

    fn enabled(&self, _ctx: &PipelineContext) -> bool {
        // @todo: enable when context management is ready
        false
    }

    async fn execute(&self, _ctx: &mut PipelineContext) -> Result<()> {
        // @todo: collect conversation history, environment, file contents
        Ok(())
    }
}
