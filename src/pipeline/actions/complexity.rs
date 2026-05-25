//! Prompt complexity classifier action.
//! Evaluates prompt complexity (0-10) and returns next actions.

use anyhow::Result;

use crate::pipeline::action::PipelineAction;
use crate::pipeline::actions::species::SpeciesAction;
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

/// Evaluates prompt complexity.
pub struct ComplexityAction;

#[async_trait::async_trait]
impl PipelineAction for ComplexityAction {
    fn name(&self) -> &'static str {
        "Complexity"
    }

    async fn execute(&self, ctx: &mut PipelineContext) -> Result<Vec<Box<dyn PipelineAction>>> {
        // @todo: real complexity evaluation
        ctx.complexity = Some(ComplexityType::Low);
        // Next: determine species
        Ok(vec![Box::new(SpeciesAction)])
    }
}
