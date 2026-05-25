//! Prompt species classifier action.
//! Determines prompt type.

use anyhow::Result;

use crate::pipeline::action::PipelineAction;
use crate::pipeline::actions::clean::CleanAction;
use crate::pipeline::context::PipelineContext;

/// Prompt species/type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeciesType {
    /// Code-related request.
    Code,
    /// General question.
    Question,
    /// System command (time, url, etc.).
    System,
    /// Context-dependent request.
    Context,
}

/// Determines prompt type.
pub struct SpeciesAction;

#[async_trait::async_trait]
impl PipelineAction for SpeciesAction {
    fn name(&self) -> &'static str {
        "Species"
    }

    async fn execute(&self, ctx: &mut PipelineContext) -> Result<Vec<Box<dyn PipelineAction>>> {
        // @todo: real species detection
        ctx.species = Some(SpeciesType::Question);
        // Next: clean prompt based on agent
        Ok(vec![Box::new(CleanAction)])
    }
}
