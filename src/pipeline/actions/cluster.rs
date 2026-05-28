//! Cluster call action.
//! Sends prompt to vibe-cluster and stores response.

use anyhow::Result;

use vibe_cluster::Prompt;

use crate::models::prompt::PromptMode;
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
        // @todo
        // if let Some(data) = &ctx.parse_action {
        //     println!("3 ==============");
        //     println!("{:?}", data);
        // }

        if let Some(data) = &ctx.parse_action {
            if data.data.mode == PromptMode::Service {
                // Store cluster result
                ctx.cluster_action = Some(ClusterActionContext {
                    response: "Задача успешно выполнена. Что делаем дальше?".to_string(),
                });
                return Ok(());
            }
        }

        // Get clean or raw prompt
        let prompt = ctx
            .parse_action
            .as_ref()
            .map(|c| c.data.make_prompt())
            .unwrap_or_else(|| ctx.prompt_raw.clone());

        // Send prompt to cluster
        let response = ctx
            .create_cluster()?
            .call(Prompt::new(prompt))
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?
            .text
            .ok_or_else(|| anyhow::anyhow!("Empty response from cluster"))?;

        // @todo
        // println!("4 ==============");
        // println!("{:?}", response);

        // Store cluster result
        ctx.cluster_action = Some(ClusterActionContext { response });

        // Action completed, returning to pipeline
        Ok(())
    }
}
