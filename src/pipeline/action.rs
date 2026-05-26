//! Pipeline action trait.
//! Each action processes context and returns next actions.

use anyhow::Result;

use crate::pipeline::context::PipelineContext;

/// Pipeline action — processes context.
#[async_trait::async_trait]
pub trait PipelineAction: Send + Sync {
    /// Action name for logging.
    fn name(&self) -> &'static str;
    /// Whether this action should run given current context.
    fn enabled(&self, _ctx: &PipelineContext) -> bool;
    /// Execute action.
    async fn execute(&self, ctx: &mut PipelineContext) -> Result<()>;
}
