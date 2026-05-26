//! Cline client formatter.
//! Formats responses as tool_calls for Cline AI.

use crate::clients::formatter::ClientFormatter;

/// Cline client formatter.
pub struct ClineFormatter;

impl ClientFormatter for ClineFormatter {
    fn format_sync(&self, response: &str) -> String {
        self.format_stream(response)
    }

    fn format_stream(&self, response: &str) -> String {
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
                            "name": "attempt_completion",
                            "arguments": format!(r#"{{"result": "{}"}}"#, response)
                        }
                    }]
                },
                "finish_reason": "tool_calls"
            }]
        });

        serde_json::to_string(&vec![chunk]).unwrap_or_else(|_| response.to_string())
    }
}
