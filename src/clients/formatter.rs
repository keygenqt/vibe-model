//! Client formatter trait.
//! Each client formats responses differently.

/// Client formatter trait.
pub trait ClientFormatter: Send + Sync {
    /// Format response text for sync response.
    fn format_sync(&self, response: &str) -> String;
    /// Format response text for stream response.
    fn format_stream(&self, response: &str) -> String;
}
