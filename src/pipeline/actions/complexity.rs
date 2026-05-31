//! Prompt complexity classifier action.
//! Evaluates prompt complexity (0-10) and returns next actions.

use anyhow::Result;
use vibe_cluster::Prompt;

use crate::pipeline::action::PipelineAction;
use crate::pipeline::context::PipelineContext;

/// Complexity action result.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ComplexityActionContext {
    /// Prompt sent to cluster for complexity estimate.
    pub prompt: String,
    /// Response text from cluster.
    pub response: String,
    /// Evaluated prompt complexity.
    pub complexity: f32,
}

/// Evaluates prompt complexity.
pub struct ComplexityAction;

#[async_trait::async_trait]
impl PipelineAction for ComplexityAction {
    /// Action name for logging.
    fn name(&self) -> &'static str {
        "Complexity"
    }

    /// Whether this action should run given current context.
    fn enabled(&self, ctx: &PipelineContext) -> bool {
        ctx.cache_action.is_none() && ctx.system_action.is_none()
    }

    /// Execute action.
    async fn execute(&self, ctx: &mut PipelineContext) -> Result<()> {
        // Get clean or raw prompt
        let prompt = ctx
            .parse_action
            .as_ref()
            .map(|c| c.user.make_prompt())
            .unwrap_or_else(|| ctx.request.user_prompt().unwrap_or_default());

        // Static system identity for cluster.
        let complexity_prompt = format!(
            r#"
Rate prompt complexity from 0.00 to 1.00 based on:
- Depth of knowledge required (shallow -> deep expertise)
- Code involvement (no code -> architecture/creative)
- Analytical demand (simple recall -> comparison/synthesis)
- Output complexity (few words -> structured document/code)

Reply ONLY with the number (two decimal places, e.g. 0.50).

<prompt>
{prompt}
</prompt>
"#,
        )
        .trim()
        .to_string();

        // Send prompt to estimator cluster
        let response = ctx
            .create_estimator()?
            .call(Prompt::new(&complexity_prompt, None))
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?
            .text
            .ok_or_else(|| anyhow::anyhow!("Empty response from estimator"))?;

        // Parse response to f32, round to 2 decimal places.
        let result: f32 = response
            .trim()
            .replace(',', ".")
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid complexity response: '{}'", response))?;

        // Clamp to 0.0-1.0 and round to 2 decimal places.
        let result = ((result.clamp(0.0, 1.0)) * 100.0).round() / 100.0;

        // Store complexity result
        ctx.complexity_action = Some(ComplexityActionContext {
            prompt: complexity_prompt,
            response: response.clone(),
            complexity: result,
        });

        // Action completed, returning to pipeline
        Ok(())
    }
}
