//! Prompt cleaning action.
//! Removes agent-specific garbage from prompt.

use anyhow::Result;
use regex::Regex;

use crate::pipeline::action::PipelineAction;
use crate::pipeline::actions::cluster::ClusterAction;
use crate::pipeline::context::PipelineContext;
use crate::providers::agents::AgentType;

/// Cleans agent-specific garbage from prompt.
pub struct CleanAction;

#[async_trait::async_trait]
impl PipelineAction for CleanAction {
    fn name(&self) -> &'static str {
        "Clean"
    }

    async fn execute(&self, ctx: &mut PipelineContext) -> Result<Vec<Box<dyn PipelineAction>>> {
        if ctx.agent == AgentType::Cline {
            ctx.prompt = clean_cline_prompt(&ctx.prompt);
        }
        // Next: send to cluster
        Ok(vec![Box::new(ClusterAction)])
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
