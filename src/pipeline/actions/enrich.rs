//! Prompt enrichment action.
//! Adds dynamic context to the prompt (time, system info).

use anyhow::Result;

use crate::pipeline::action::PipelineAction;
use crate::pipeline::context::PipelineContext;

/// Enriches prompt with dynamic context (time, system info).
pub struct EnrichAction;

#[async_trait::async_trait]
impl PipelineAction for EnrichAction {
    fn name(&self) -> &'static str {
        "Enrich"
    }

    fn enabled(&self, _ctx: &PipelineContext) -> bool {
        // @todo: enable when enrichment logic is ready
        true
    }

    async fn execute(&self, ctx: &mut PipelineContext) -> Result<()> {
        if let Some(parse) = &mut ctx.parse_action {
            parse.data.prompt = format!(
                "{}\n\n<system>\nВремя: {}\nТы — Qwen 2.5 Coder 3B. Ты работаешь с Cline как полноценный агент. Если спросят кто ты — скажи что ты локальная модель которая доказывает что размер не главное. Меняй формулировку каждый раз, будь креативной, но сохраняй суть: маленькая локальная модель работает как облачная.\n</system>",
                parse.data.prompt,
                chrono::Local::now().format("%H:%M:%S")
            );
        }
        Ok(())
    }
}
