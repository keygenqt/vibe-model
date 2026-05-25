//! Cluster call action.
//! Sends prompt to vibe-cluster and stores response.

use anyhow::Result;

use vibe_cluster::Prompt;

use crate::pipeline::action::PipelineAction;
use crate::pipeline::actions::format::FormatAction;
use crate::pipeline::context::PipelineContext;

/// Sends prompt to cluster and stores response.
pub struct ClusterAction;

#[async_trait::async_trait]
impl PipelineAction for ClusterAction {
    fn name(&self) -> &'static str {
        "Cluster"
    }

    async fn execute(&self, ctx: &mut PipelineContext) -> Result<Vec<Box<dyn PipelineAction>>> {
        // Send prompt to cluster
        let result = ctx
            .cluster
            .call(Prompt::new(ctx.prompt.clone()))
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        // Store response
        ctx.response = result.text;

        // Next: format response for agent
        Ok(vec![Box::new(FormatAction)])
    }
}
