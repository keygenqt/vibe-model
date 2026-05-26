//! System action for built-in responses.
//! Handles system queries without calling the model.

use anyhow::Result;
use chrono::Local;

use crate::pipeline::action::PipelineAction;
use crate::pipeline::actions::species::SpeciesType;
use crate::pipeline::context::PipelineContext;

/// System action result.
#[derive(Debug, Clone)]
pub struct SystemActionContext {
    /// System response text.
    pub response: String,
}

/// Handles system queries (time, etc.).
pub struct SystemAction;

#[async_trait::async_trait]
impl PipelineAction for SystemAction {
    fn name(&self) -> &'static str {
        "System"
    }

    /// Whether this action should run given current context.
    fn enabled(&self, ctx: &PipelineContext) -> bool {
        ctx.species_action
            .as_ref()
            .map(|a| a.species == SpeciesType::System)
            .unwrap_or(false)
    }

    /// Execute action.
    async fn execute(&self, ctx: &mut PipelineContext) -> Result<()> {
        if ctx.prompt_raw.to_lowercase().contains("сколько время") {
            ctx.system_action = Some(SystemActionContext {
                response: get_time(),
            });
        }
        Ok(())
    }
}

/// Get current system time.
fn get_time() -> String {
    let now = Local::now();
    format!("Текущее время: {}", now.format("%H:%M:%S"))
}
