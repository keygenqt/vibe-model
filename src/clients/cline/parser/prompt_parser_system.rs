//! Cline prompt parser.
//! Extracts task, file content, and feedback from Cline prompts.

use crate::{
    clients::parsers::PromptParserSystem, models::prompt_system::PromptSystemModel, utils::parse,
};

/// Parses Cline-specific prompt format.
pub struct ClinePromptParserSystem;

impl PromptParserSystem for ClinePromptParserSystem {
    fn parse(&self, prompt: &str) -> PromptSystemModel {
        let mut system = PromptSystemModel::new();

        // 1. Collect all tags into one array
        let tags = vec![
            "USER'S CUSTOM INSTRUCTIONS",
            "SKILLS",
            "MCP SERVERS",
            "RULES",
            "====",
        ];

        // 2. Single pass over text to get serde_json::Value
        let mut json_data = parse::extract_sections_by_tags(prompt, &tags);

        // 3. Extract data from JSON object and map to model
        system.instructions = json_data
            .get_mut("USER'S CUSTOM INSTRUCTIONS")
            .and_then(|v| v.take().as_str().map(|s| s.to_string()));

        system.skills = json_data
            .get_mut("SKILLS")
            .and_then(|v| v.take().as_str().map(|s| s.to_string()));

        system.mcp_servers = json_data
            .get_mut("MCP SERVERS")
            .and_then(|v| v.take().as_str().map(|s| s.to_string()));

        system.rules = json_data
            .get_mut("RULES")
            .and_then(|v| v.take().as_str().map(|s| s.to_string()));

        // 4. Filter and extract core rules
        system.rules = system
            .rules
            .map(|rules| parse::extract_caps_lines(&rules, true, &["replace_in_file"]));

        // Done
        system
    }
}
