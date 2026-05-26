//! Format response action.
//! Formats model response and stream chunks based on agent type.

use anyhow::Result;

use crate::clients::cline::formatter::ClineFormatter;
use crate::clients::formatter::ClientFormatter;
use crate::pipeline::action::PipelineAction;
use crate::pipeline::context::PipelineContext;
use crate::providers::agents::AgentType;
use crate::providers::openai::formatter::OpenAIFormatter;

/// Format action result.
#[derive(Debug, Clone)]
pub struct FormatActionContext {
    /// Formatted response text (sync clients).
    pub response: Option<String>,
    /// Stream chunks (streaming clients).
    pub stream_chunks: Option<String>,
}

/// Formats response for specific agent.
pub struct FormatAction;

#[async_trait::async_trait]
impl PipelineAction for FormatAction {
    /// Action name for logging.
    fn name(&self) -> &'static str {
        "Format"
    }

    /// Whether this action should run given current context.
    fn enabled(&self, _ctx: &PipelineContext) -> bool {
        true
    }

    /// Execute action.
    async fn execute(&self, ctx: &mut PipelineContext) -> Result<()> {
        // Get response from cache, system, or cluster
        let response = ctx.get_response()?;

        // Select formatter by agent type
        let formatter: Box<dyn ClientFormatter> = match ctx.agent {
            AgentType::Cline => Box::new(ClineFormatter),
            _ => Box::new(OpenAIFormatter),
        };

        // Format and store
        ctx.format_action = Some(FormatActionContext {
            response: Some(formatter.format_sync(&response)),
            stream_chunks: Some(formatter.format_stream(&response)),
        });

        // Formatting completed
        Ok(())
    }
}
