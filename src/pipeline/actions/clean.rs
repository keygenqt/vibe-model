//! Prompt cleaning action.
//! Removes agent-specific garbage from prompt.

use anyhow::Result;
use regex::Regex;

use crate::pipeline::action::PipelineAction;
use crate::pipeline::actions::complexity::ComplexityType;
use crate::pipeline::context::PipelineContext;
use crate::providers::agents::AgentType;

/// Clean action result.
#[derive(Debug, Clone)]
pub struct CleanActionContext {
    /// Cleaned prompt text.
    pub prompt: String,
}

/// Cleans agent-specific garbage from prompt.
pub struct CleanAction;

#[async_trait::async_trait]
impl PipelineAction for CleanAction {
    /// Action name for logging.
    fn name(&self) -> &'static str {
        "Clean"
    }

    /// Whether this action should run given current context.
    fn enabled(&self, ctx: &PipelineContext) -> bool {
        ctx.cache_action.is_none() && ctx.system_action.is_none()
    }

    /// Execute action.
    async fn execute(&self, ctx: &mut PipelineContext) -> Result<()> {
        // Disable clean for low complexity
        if let Some(complexity_action) = &ctx.complexity_action {
            if complexity_action.complexity == ComplexityType::Low {
                ctx.clean_action = Some(CleanActionContext {
                    prompt: ctx.prompt_raw.clone(),
                });
                return Ok(());
            }
        }

        // Action
        let prompt = if ctx.agent == AgentType::Cline {
            clean_cline_prompt(&ctx.prompt_raw)
        } else {
            ctx.prompt_raw.clone()
        };

        // Store clean result
        ctx.clean_action = Some(CleanActionContext { prompt });

        // Action completed, returning to pipeline
        Ok(())
    }
}

/// Remove Cline-specific garbage from prompt, keeping only task and file content.
fn clean_cline_prompt(prompt: &str) -> String {
    // Extract <task> block
    let task_re = Regex::new(r"(?s)<task>(.*?)</task>").unwrap();
    let task = task_re
        .captures(prompt)
        .map(|c| format!("<task>{}</task>", c[1].trim()))
        .unwrap_or_default();

    // Extract <file_content> block
    let file_re = Regex::new(r"(?s)<file_content[^>]*>(.*?)</file_content>").unwrap();
    let file = file_re
        .captures(prompt)
        .map(|c| format!("<file_content>{}</file_content>", c[1].trim()))
        .unwrap_or_default();

    // Extract <feedback> block if present
    let feedback_re = Regex::new(r"(?s)<feedback>(.*?)</feedback>").unwrap();
    let feedback = feedback_re
        .captures(prompt)
        .map(|c| format!("<feedback>{}</feedback>", c[1].trim()))
        .unwrap_or_default();

    let mut parts = Vec::new();
    if !task.is_empty() {
        parts.push(task);
    }
    if !file.is_empty() {
        parts.push(file);
    }
    if !feedback.is_empty() {
        parts.push(feedback);
    }

    parts.join("\n\n")
}
