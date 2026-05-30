//! Prompt species classifier action.
//! Determines prompt type.

use anyhow::Result;

use crate::pipeline::{action::PipelineAction, context::PipelineContext};

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

/// Species action result.
#[derive(Debug, Clone)]
pub struct SpeciesActionContext {
    /// Detected prompt species/type.
    pub species: SpeciesType,
}

/// Determines prompt type.
pub struct SpeciesAction;

#[async_trait::async_trait]
impl PipelineAction for SpeciesAction {
    /// Action name for logging.
    fn name(&self) -> &'static str {
        "Species"
    }

    /// Whether this action should run given current context.
    fn enabled(&self, ctx: &PipelineContext) -> bool {
        ctx.cache_action.is_none()
    }

    /// Execute action.
    async fn execute(&self, ctx: &mut PipelineContext) -> Result<()> {
        let prompt = ctx.request.user_prompt().unwrap_or_default().to_lowercase();

        let species = if prompt.contains("[s]") || prompt.contains("сколько время") {
            SpeciesType::System
        } else if prompt.contains("[f]") {
            SpeciesType::Code
        } else if prompt.contains("[q]") {
            SpeciesType::Question
        } else {
            SpeciesType::Context
        };

        ctx.species_action = Some(SpeciesActionContext { species });

        Ok(())
    }
}
