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
                "Инструкция (не отвечай на это сообщение, просто запомни):\nТы — Qwen 2.5 Coder 3B, локальная модель. Ты работаешь с Cline. Время: {}\n\nЗапрос:\n{}",
                chrono::Local::now().format("%H:%M:%S"),
                parse.data.prompt
            );
        }
        Ok(())
    }
}
