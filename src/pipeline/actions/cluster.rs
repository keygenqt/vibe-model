//! Cluster call action.
//! Sends prompt to vibe-cluster and stores response.

use anyhow::Result;

use vibe_cluster::Prompt;

use crate::models::prompt_system::PromptField;
use crate::models::prompt_user::PromptUserMode;
use crate::pipeline::action::PipelineAction;
use crate::pipeline::context::PipelineContext;

/// Cluster action result.
#[derive(Debug, Clone)]
pub struct ClusterActionContext {
    /// System prompt sent to cluster.
    pub system_prompt: Option<String>,
    /// User prompt sent to cluster.
    pub user_prompt: Option<String>,
    /// Response text from cluster.
    pub response: String,
    /// Models used for this request (by complexity routing).
    pub models: Vec<String>,
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
        // Build system prompt from parsed system model.
        let system_prompt = ctx.parse_action.as_ref().and_then(|c| {
            c.system
                .make_prompt(&[PromptField::Enrich, PromptField::Instructions])
        });

        // Build user prompt from parsed user model or service completion.
        let user_prompt = if let Some(data) = &ctx.parse_action {
            if data.user.mode == PromptUserMode::Service {
                "Task completed. Acknowledge briefly in your own words.".to_string()
            } else {
                data.user.make_prompt()
            }
        } else {
            ctx.request.user_prompt().unwrap_or_default()
        };

        // Get complexity score from estimator or fallback to 0.5.
        let complexity = ctx
            .complexity_action
            .as_ref()
            .map(|c| c.complexity)
            .unwrap_or(0.5);

        // Create cluster filtered by complexity score.
        let cluster = ctx.create_cluster(complexity)?;

        // Send prompt to cluster filtered by complexity
        let response = cluster
            .call(Prompt::new(&user_prompt, system_prompt.clone()))
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?
            .text
            .ok_or_else(|| anyhow::anyhow!("Empty response from cluster"))?;

        let response = response
            .trim_matches(|c: char| c == '-' || c.is_whitespace())
            .to_string();

        // Store cluster result
        ctx.cluster_action = Some(ClusterActionContext {
            system_prompt: system_prompt,
            user_prompt: Some(user_prompt),
            response,
            models: cluster.models().await,
        });

        // Action completed, returning to pipeline
        Ok(())
    }
}
