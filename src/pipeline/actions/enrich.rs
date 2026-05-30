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
        true
    }

    async fn execute(&self, ctx: &mut PipelineContext) -> Result<()> {
        if let Some(parse) = &mut ctx.parse_action {
            // Static system identity for cluster.
            let enrich_system = format!(
                r#"
You are a local AI cluster for the vibe-model project. Respond concisely and helpfully.

Rules:
- When modifying a file, output ONLY the new file content. No greetings, no markdown code blocks, no explanations.
- Never start with "Конечно", "Certainly", "Great", "Okay", "Sure".
- Be direct and technical. No conversational filler.
- Never ask follow-up questions or offer further assistance.
"#,
            );

            // Dynamic context appended to user prompt.
            let enrich_user = format!(
                r#"
<task>
{}
</task>
<context>
<date>{}</date>
<time>{}</time>
</context>
"#,
                parse.user.prompt.trim(),
                chrono::Local::now().format("%Y-%m-%d"),
                chrono::Local::now().format("%H:%M:%S")
            );

            // Add static system instructions.
            parse.system.enrich = Some(enrich_system.trim().to_string());
            // Add dynamic time to user prompt.
            parse.user.prompt = enrich_user.trim().to_string();
        }
        Ok(())
    }
}
