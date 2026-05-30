//! Parse action.
//! Parses raw prompt into structured PromptModel.

use anyhow::Result;

use crate::clients::cline::parser::prompt_parser_system::ClinePromptParserSystem;
use crate::clients::cline::parser::prompt_parser_user::ClinePromptParserUser;
use crate::clients::parsers::{
    BasePromptParserSystem, BasePromptParserUser, PromptParserSystem, PromptParserUser,
};
use crate::models::prompt_system::PromptSystemModel;
use crate::models::prompt_user::PromptUserModel;
use crate::pipeline::action::PipelineAction;
use crate::pipeline::context::PipelineContext;
use crate::providers::agents::AgentType;

/// Parse action result.
#[derive(Debug, Clone)]
pub struct ParseActionContext {
    /// Parsed system prompt data.
    pub system: PromptSystemModel,
    /// Parsed prompt data.
    pub user: PromptUserModel,
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
        let system_prompt = ctx.request.system_prompt().unwrap_or_default();
        let user_prompt = ctx.request.user_prompt().unwrap_or_default();

        let parser_system: Box<dyn PromptParserSystem> = match ctx.agent {
            AgentType::Cline => Box::new(ClinePromptParserSystem),
            _ => Box::new(BasePromptParserSystem),
        };

        let parser_user: Box<dyn PromptParserUser> = match ctx.agent {
            AgentType::Cline => Box::new(ClinePromptParserUser),
            _ => Box::new(BasePromptParserUser),
        };

        ctx.parse_action = Some(ParseActionContext {
            system: parser_system.parse(&system_prompt),
            user: parser_user.parse(&user_prompt),
        });

        Ok(())
    }
}
