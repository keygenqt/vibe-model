//! Pipeline action trait.
//! Each action processes context and returns next actions.

use anyhow::Result;

use crate::pipeline::context::PipelineContext;

/// Pipeline action — processes context and returns next actions.
/// Empty vec means pipeline is done.
#[async_trait::async_trait]
pub trait PipelineAction: Send + Sync {
    /// Action name for logging.
    fn name(&self) -> &'static str;
    /// Execute action and return next actions to run.
    async fn execute(&self, ctx: &mut PipelineContext) -> Result<Vec<Box<dyn PipelineAction>>>;
}
