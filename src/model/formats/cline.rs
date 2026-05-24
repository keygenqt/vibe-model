//! Cline tool calls response format.
//! Formats responses as tool calls for Cline AI.

use super::ResponseFormat;
use crate::model::types::request::{FunctionCall, Message, ToolCall};

/// Cline tool calls format handler.
pub struct ClineFormat;

impl ResponseFormat for ClineFormat {
    fn format_message(&self, content: &str) -> Message {
        Message {
            role: "assistant".into(),
            content: None,
            tool_calls: Some(vec![ToolCall {
                id: format!("call_{}", uuid::Uuid::new_v4()),
                function: FunctionCall {
                    name: "attempt_completion".into(),
                    arguments: format!(r#"{{"result": "{}"}}"#, content),
                },
            }]),
            tool_call_id: None,
        }
    }

    fn clean_prompt(&self, prompt: &str) -> String {
        clean_cline_prompt(prompt)
    }
}

/// Remove Cline-specific garbage from prompt, keeping only task and file content.
fn clean_cline_prompt(prompt: &str) -> String {
    // Extract <task> block
    let task_re = regex::Regex::new(r"(?s)<task>(.*?)</task>").unwrap();
    let task = task_re
        .captures(prompt)
        .map(|c| format!("<task>{}</task>", c[1].trim()))
        .unwrap_or_default();

    // Extract <file_content> block
    let file_re = regex::Regex::new(r"(?s)<file_content[^>]*>(.*?)</file_content>").unwrap();
    let file = file_re
        .captures(prompt)
        .map(|c| format!("<file_content>{}</file_content>", c[1].trim()))
        .unwrap_or_default();

    // Extract <feedback> block if present
    let feedback_re = regex::Regex::new(r"(?s)<feedback>(.*?)</feedback>").unwrap();
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
