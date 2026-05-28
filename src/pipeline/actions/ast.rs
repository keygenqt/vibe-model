//! AST enrichment action.
//! Adds code structure from vibe-ast to the prompt.

use anyhow::Result;

use crate::pipeline::action::PipelineAction;
use crate::pipeline::context::PipelineContext;

/// Enriches prompt with AST data from vibe-ast.
pub struct AstAction;

#[async_trait::async_trait]
impl PipelineAction for AstAction {
    fn name(&self) -> &'static str {
        "AST"
    }

    fn enabled(&self, _ctx: &PipelineContext) -> bool {
        // @todo: enable when AST integration is ready
        false
    }

    async fn execute(&self, _ctx: &mut PipelineContext) -> Result<()> {
        // @todo: parse files with vibe-ast, add structure to prompt
        Ok(())
    }
}
