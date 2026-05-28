//! Client formatter trait.
//! Each client formats responses differently.

use crate::pipeline::context::PipelineContext;

/// Client formatter trait.
pub trait ClientFormatter: Send + Sync {
    /// Format response text for sync response.
    fn format_sync(&self, ctx: &PipelineContext, response: &str) -> String;
    /// Format response text for stream response.
    fn format_stream(&self, ctx: &PipelineContext, response: &str) -> String;
}
