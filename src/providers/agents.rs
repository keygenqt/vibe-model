/// Known AI agent types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentType {
    /// Generic/unknown client.
    Unknown,
    /// Cline AI.
    Cline,
    /// Open WebUI.
    OpenWebUI,
    /// Continue.dev.
    Continue,
    /// Aider.
    Aider,
}

impl AgentType {
    /// Detect agent type from user-agent header.
    pub fn from_user_agent(user_agent: &str) -> Self {
        if user_agent.contains("Cline") {
            AgentType::Cline
        } else if user_agent.contains("Continue") {
            AgentType::Continue
        } else if user_agent.contains("Aider") {
            AgentType::Aider
        } else if user_agent.contains("Python/3.") || user_agent.contains("aiohttp") {
            AgentType::OpenWebUI
        } else {
            AgentType::Unknown
        }
    }
}
