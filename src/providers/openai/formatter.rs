//! Common text client formatter.
//! Formats responses as plain text for generic clients.

use crate::{clients::formatter::ClientFormatter, pipeline::context::PipelineContext};

/// Text client formatter.
pub struct OpenAIFormatter;

impl ClientFormatter for OpenAIFormatter {
    fn format_sync(&self, _ctx: &PipelineContext, response: &str) -> String {
        serde_json::to_string(&serde_json::json!({
            "id": format!("chatcmpl-{}", uuid::Uuid::new_v4()),
            "object": "chat.completion",
            "created": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            "model": "vibe-model",
            "choices": [{
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": response
                },
                "finish_reason": "stop"
            }]
        }))
        .unwrap_or_else(|_| response.to_string())
    }

    fn format_stream(&self, _ctx: &PipelineContext, response: &str) -> String {
        let id = format!("chatcmpl-{}", uuid::Uuid::new_v4());
        let created = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let words: Vec<&str> = response.split_whitespace().collect();
        let mut chunks = Vec::new();

        // First chunk with role
        if !words.is_empty() {
            chunks.push(serde_json::json!({
                "id": id,
                "object": "chat.completion.chunk",
                "created": created,
                "model": "vibe-model",
                "choices": [{
                    "index": 0,
                    "delta": {"role": "assistant"},
                    "finish_reason": null
                }]
            }));
        }

        // Content chunks
        for word in &words {
            chunks.push(serde_json::json!({
                "id": id,
                "object": "chat.completion.chunk",
                "created": created,
                "model": "vibe-model",
                "choices": [{
                    "index": 0,
                    "delta": {"content": format!("{} ", word)},
                    "finish_reason": null
                }]
            }));
        }

        // Final chunk
        chunks.push(serde_json::json!({
            "id": id,
            "object": "chat.completion.chunk",
            "created": created,
            "model": "vibe-model",
            "choices": [{
                "index": 0,
                "delta": {},
                "finish_reason": "stop"
            }]
        }));

        serde_json::to_string(&chunks).unwrap_or_else(|_| response.to_string())
    }
}
