//! Cline client formatter.
//! Formats responses as tool_calls for Cline AI.

use crate::{clients::formatter::ClientFormatter, pipeline::context::PipelineContext};

/// Cline client formatter.
pub struct ClineFormatterOpenAI;

impl ClientFormatter for ClineFormatterOpenAI {
    fn format_sync(&self, ctx: &PipelineContext, response: &str) -> String {
        self.format_stream(ctx, response)
    }

    fn format_stream(&self, ctx: &PipelineContext, response: &str) -> String {
        // Determine tool based on parsed data
        let (tool_name, tool_args) = if let Some(parse_ctx) = &ctx.parse_action {
            if let Some(file) = parse_ctx.data.files.first() {
                // File modification task — suggest write_to_file
                (
                    "write_to_file",
                    format!(r#"{{"path": "{}", "content": "{}"}}"#, file.path, response),
                )
            } else {
                // No files — attempt_completion
                (
                    "attempt_completion",
                    format!(r#"{{"result": "{}"}}"#, response),
                )
            }
        } else {
            (
                "attempt_completion",
                format!(r#"{{"result": "{}"}}"#, response),
            )
        };

        let chunk = serde_json::json!({
            "id": format!("chatcmpl-{}", uuid::Uuid::new_v4()),
            "object": "chat.completion.chunk",
            "created": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            "model": "vibe-model",
            "choices": [{
                "index": 0,
                "delta": {
                    "role": "assistant",
                    "tool_calls": [{
                        "id": format!("call_{}", uuid::Uuid::new_v4()),
                        "type": "function",
                        "function": {
                            "name": tool_name,
                            "arguments": tool_args
                        }
                    }]
                },
                "finish_reason": "tool_calls"
            }]
        });

        serde_json::to_string(&vec![chunk]).unwrap_or_else(|_| response.to_string())
    }
}
