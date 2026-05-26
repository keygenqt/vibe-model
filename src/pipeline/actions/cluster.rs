//! Cluster call action.
//! Sends prompt to vibe-cluster and stores response.

use anyhow::Result;

use vibe_cluster::Prompt;

use crate::pipeline::action::PipelineAction;
use crate::pipeline::context::PipelineContext;

/// Cluster action result.
#[derive(Debug, Clone)]
pub struct ClusterActionContext {
    /// Response text from cluster.
    pub response: String,
}

/// Sends prompt to cluster and stores response.
pub struct ClusterAction;

#[async_trait::async_trait]
impl PipelineAction for ClusterAction {
    /// Action name for logging.
    fn name(&self) -> &'static str {
        "Cluster"
    }

    /// Whether this action should run given current context.
    fn enabled(&self, ctx: &PipelineContext) -> bool {
        ctx.cache_action.is_none() && ctx.system_action.is_none()
    }

    /// Execute action.
    async fn execute(&self, ctx: &mut PipelineContext) -> Result<()> {
        // Get clean or raw prompt
        let prompt = ctx
            .clean_action
            .as_ref()
            .map(|c| c.prompt.clone())
            .unwrap_or_else(|| ctx.prompt_raw.clone());

        // Send prompt to cluster
        let response = ctx
            .create_cluster()?
            .call(Prompt::new(prompt))
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?
            .text
            .ok_or_else(|| anyhow::anyhow!("Empty response from cluster"))?;

        // Store cluster result
        ctx.cluster_action = Some(ClusterActionContext { response });

        // Action completed, returning to pipeline
        Ok(())
    }
}
