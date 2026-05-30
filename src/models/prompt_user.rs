//! Prompt user model — structured prompt data.

use crate::models::cline_context::ClineContextModel;

/// Prompt mode.
#[derive(Debug, Clone, PartialEq)]
pub enum PromptUserMode {
    /// New task from user.
    Task,
    /// Service message from Cline (tool result, error).
    Service,
    /// Unknown prompt type.
    Unknown,
}

/// Parsed prompt model.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PromptUserModel {
    /// User prompt text.
    pub prompt: String,
    /// Prompt mode.
    pub mode: PromptUserMode,
    /// Files attached to the task.
    pub files: Vec<PromptUserFile>,
    /// Cline-specific data (environment, terminal, diagnostics).
    pub cline_context: Option<ClineContextModel>,
}

/// File from client prompt.
#[derive(Debug, Clone)]
pub struct PromptUserFile {
    /// File path.
    pub path: String,
    /// File content.
    pub content: String,
}

impl PromptUserModel {
    /// Create PromptUserModel from task string.
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            mode: PromptUserMode::Unknown,
            files: vec![],
            cline_context: None,
        }
    }

    /// Build prompt text for model including files.
    pub fn make_prompt(&self) -> String {
        if self.files.is_empty() {
            return self.prompt.trim().to_string();
        }
        self.make_prompt_with_files()
    }

    /// Build prompt with file contents appended.
    fn make_prompt_with_files(&self) -> String {
        let mut prompt = self.prompt.clone();
        for file in &self.files {
            prompt.push_str(&format!("\n\n--- {} ---\n{}", file.path, file.content));
        }
        prompt
    }
}
