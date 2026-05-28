//! Prompt XML model for deserialization.

use serde::Deserialize;

/// Clean structure mapped directly from the extracted JSON fields.
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct PromptTagsModel {
    /// Task from <task> tag.
    pub task: Option<String>,
    /// Task from <feedback> tag.
    pub feedback: Option<String>,
    /// Environment details text.
    pub environment_details: Option<String>,
    /// Terminal output from <terminal_output> tag.
    pub terminal_output: Option<String>,
    /// Workspace diagnostics from <workspace_diagnostics> tag.
    pub workspace_diagnostics: Option<String>,
    /// Final file content after edits from <final_file_content>.
    pub final_file_content: Option<String>,
    /// Files from <file_content> tags.
    #[serde(default)]
    pub file_content: Vec<PromptTagsFile>,
}

/// File fields mapping to JSON keys "path" and "content".
#[derive(Debug, Clone, Deserialize)]
pub struct PromptTagsFile {
    /// File path from `path` attribute.
    pub path: String,
    /// File content from tag body.
    pub content: String,
}

impl PromptTagsModel {
    /// Get prompt from task, feedback, or context.
    pub fn get_user_prompt(&self, context: &Option<String>) -> String {
        self.task
            .clone()
            .or_else(|| self.feedback.clone())
            .or_else(|| context.clone())
            .unwrap_or_else(|| "No task specified".to_string())
    }
}
