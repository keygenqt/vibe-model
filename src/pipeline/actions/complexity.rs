//! Prompt complexity classifier action.
//! Evaluates prompt complexity (0-10) and returns next actions.

use anyhow::Result;

use crate::pipeline::action::PipelineAction;
use crate::pipeline::context::PipelineContext;

/// Known AI agent types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexityType {
    /// Low complexity.
    Low,
    /// Medium complexity.
    Medium,
    /// High complexity.
    High,
}

/// Complexity action result.
#[derive(Debug, Clone)]
pub struct ComplexityActionContext {
    /// Evaluated prompt complexity.
    pub complexity: ComplexityType,
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
            .clean_action
            .as_ref()
            .map(|c| c.prompt.clone())
            .unwrap_or_else(|| ctx.prompt_raw.clone());

        let complexity = if prompt.len() < 200 {
            ComplexityType::Low
        } else if prompt.len() < 1000 {
            ComplexityType::Medium
        } else {
            ComplexityType::High
        };

        // Store complexity result
        ctx.complexity_action = Some(ComplexityActionContext { complexity });

        // Action completed, returning to pipeline
        Ok(())
    }
}
