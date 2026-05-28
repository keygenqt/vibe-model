//! Parse action.
//! Parses raw prompt into structured PromptModel.

use anyhow::Result;

use crate::clients::cline::parser::parser::ClineParser;
use crate::clients::parser::{BaseParser, ClientParser};
use crate::models::prompt::PromptModel;
use crate::pipeline::action::PipelineAction;
use crate::pipeline::context::PipelineContext;
use crate::providers::agents::AgentType;

/// Parse action result.
#[derive(Debug, Clone)]
pub struct ParseActionContext {
    /// Parsed prompt data.
    pub data: PromptModel,
}

/// Parses raw prompt into structured model.
pub struct ParseAction;

#[async_trait::async_trait]
impl PipelineAction for ParseAction {
    /// Action name for logging.
    fn name(&self) -> &'static str {
        "Parse"
    }

    /// Whether this action should run given current context.
    fn enabled(&self, ctx: &PipelineContext) -> bool {
        ctx.cache_action.is_none() && ctx.system_action.is_none()
    }

    /// Execute action.
    async fn execute(&self, ctx: &mut PipelineContext) -> Result<()> {
        let parser: Box<dyn ClientParser> = match ctx.agent {
            AgentType::Cline => Box::new(ClineParser),
            _ => Box::new(BaseParser),
        };

        ctx.parse_action = Some(ParseActionContext {
            data: parser.parse(&ctx.prompt_raw),
        });

        // Cleaning completed
        Ok(())
    }
}
