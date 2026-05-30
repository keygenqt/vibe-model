//! Client prompt parsers.
//! Each client provides user prompt and system prompt parsers.

use crate::models::{prompt_system::PromptSystemModel, prompt_user::PromptUserModel};

/// Parses user prompt (task, files, environment).
pub trait PromptParserUser: Send + Sync {
    /// Parse user prompt into structured model.
    fn parse(&self, prompt: &str) -> PromptUserModel;
}

/// Default user prompt parser — returns prompt as-is.
pub struct BasePromptParserUser;

impl PromptParserUser for BasePromptParserUser {
    fn parse(&self, prompt: &str) -> PromptUserModel {
        PromptUserModel::new(prompt)
    }
}

/// Parses system prompt (rules, skills, MCP, identity).
pub trait PromptParserSystem: Send + Sync {
    /// Parse system prompt into structured model.
    fn parse(&self, prompt: &str) -> PromptSystemModel;
}

/// Default system prompt parser — returns prompt as-is.
pub struct BasePromptParserSystem;

impl PromptParserSystem for BasePromptParserSystem {
    fn parse(&self, _prompt: &str) -> PromptSystemModel {
        PromptSystemModel::new()
    }
}
