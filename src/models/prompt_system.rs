//! Prompt system model — structured system prompt data.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PromptField {
    Enrich,
    Instructions,
    Skills,
    McpServers,
    Rules,
}

/// Parsed system prompt model.
#[derive(Debug, Clone)]
pub struct PromptSystemModel {
    /// Dynamic context added by EnrichAction.
    pub enrich: Option<String>,
    /// Raw USER'S CUSTOM INSTRUCTIONS section.
    pub instructions: Option<String>,
    /// Raw SKILLS section.
    pub skills: Option<String>,
    /// Raw MCP SERVERS section.
    pub mcp_servers: Option<String>,
    /// Raw RULES section.
    pub rules: Option<String>,
}

impl PromptSystemModel {
    /// Create empty system model.
    pub fn new() -> Self {
        Self {
            enrich: None,
            instructions: None,
            skills: None,
            mcp_servers: None,
            rules: None,
        }
    }

    /// Build full system prompt with all fields.
    pub fn make_full_prompt(&self) -> Option<String> {
        self.make_prompt(&[
            PromptField::Enrich,
            PromptField::Instructions,
            PromptField::Skills,
            PromptField::McpServers,
            PromptField::Rules,
        ])
    }

    /// Build system prompt text for model with optional field filter.
    pub fn make_prompt(&self, fields: &[PromptField]) -> Option<String> {
        let mut parts = Vec::new();
        if fields.contains(&PromptField::Enrich) {
            if let Some(enrich) = &self.enrich {
                parts.push(format!("System instructions:\n{}", enrich));
            }
        }
        if fields.contains(&PromptField::Instructions) {
            if let Some(instructions) = &self.instructions {
                parts.push(format!("User instructions:\n{}", instructions));
            }
        }
        if fields.contains(&PromptField::Skills) {
            if let Some(skills) = &self.skills {
                parts.push(format!("Skills:\n{}", skills));
            }
        }
        if fields.contains(&PromptField::McpServers) {
            if let Some(mcp) = &self.mcp_servers {
                parts.push(format!("MCP Servers:\n{}", mcp));
            }
        }
        if fields.contains(&PromptField::Rules) {
            if let Some(rules) = &self.rules {
                parts.push(format!("Rules:\n{}", rules));
            }
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join("\n\n"))
        }
    }
}
