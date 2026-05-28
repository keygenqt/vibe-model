//! Client parser trait.
//! Each client parses prompts into structured model.

use crate::models::prompt::PromptModel;

/// Client parser trait.
pub trait ClientParser: Send + Sync {
    /// Parse raw prompt into structured PromptModel.
    fn parse(&self, prompt: &str) -> PromptModel;
}

/// Default parser — returns prompt as-is.
pub struct BaseParser;

impl ClientParser for BaseParser {
    fn parse(&self, prompt: &str) -> PromptModel {
        PromptModel::new(prompt)
    }
}
