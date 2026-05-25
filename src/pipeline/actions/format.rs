//! Format response action.
//! Formats model response and stream chunks based on agent type.

use anyhow::Result;

use crate::pipeline::action::PipelineAction;
use crate::pipeline::context::PipelineContext;
use crate::providers::agents::AgentType;
use crate::providers::openai::request::tool::{FunctionCallRequest, ToolCallRequest};
use crate::providers::openai::response::stream::{
    ChatCompletionChunkResponse, ChoiceDeltaResponse, DeltaResponse,
};

/// Formats response for specific agent.
pub struct FormatAction;

#[async_trait::async_trait]
impl PipelineAction for FormatAction {
    fn name(&self) -> &'static str {
        "Format"
    }

    async fn execute(&self, ctx: &mut PipelineContext) -> Result<Vec<Box<dyn PipelineAction>>> {
        if let Some(response) = ctx.response.take() {
            match ctx.agent {
                AgentType::Cline => {
                    ctx.stream_chunks = Some(vec![format_cline_chunk(&response)]);
                }
                _ => {
                    ctx.response = Some(response.clone());
                    ctx.stream_chunks = Some(build_text_chunks(&response));
                }
            }
        }
        Ok(vec![])
    }
}

/// Build a single Cline stream chunk with tool_call.
fn format_cline_chunk(response: &str) -> ChatCompletionChunkResponse {
    let id = format!("chatcmpl-{}", uuid::Uuid::new_v4());
    let created = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    ChatCompletionChunkResponse {
        id,
        object: "chat.completion.chunk".into(),
        created,
        model: "vibe-model".into(),
        choices: vec![ChoiceDeltaResponse {
            index: 0,
            delta: DeltaResponse {
                role: Some("assistant".into()),
                content: None,
                tool_calls: Some(vec![ToolCallRequest {
                    id: format!("call_{}", uuid::Uuid::new_v4()),
                    function: FunctionCallRequest {
                        name: "attempt_completion".into(),
                        arguments: format!(r#"{{"result": "{}"}}"#, response),
                    },
                }]),
            },
            finish_reason: Some("tool_calls".into()),
        }],
        usage: None,
    }
}

/// Build plain text stream chunks.
fn build_text_chunks(content: &str) -> Vec<ChatCompletionChunkResponse> {
    let id = format!("chatcmpl-{}", uuid::Uuid::new_v4());
    let created = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut chunks = Vec::new();
    let words: Vec<&str> = content.split_whitespace().collect();

    if !words.is_empty() {
        chunks.push(ChatCompletionChunkResponse {
            id: id.clone(),
            object: "chat.completion.chunk".into(),
            created,
            model: "vibe-model".into(),
            choices: vec![ChoiceDeltaResponse {
                index: 0,
                delta: DeltaResponse {
                    role: Some("assistant".into()),
                    content: None,
                    tool_calls: None,
                },
                finish_reason: None,
            }],
            usage: None,
        });

        for (i, word) in words.iter().enumerate() {
            let partial = if i == 0 {
                word.to_string()
            } else {
                format!(" {}", word)
            };
            chunks.push(ChatCompletionChunkResponse {
                id: id.clone(),
                object: "chat.completion.chunk".into(),
                created,
                model: "vibe-model".into(),
                choices: vec![ChoiceDeltaResponse {
                    index: 0,
                    delta: DeltaResponse {
                        role: None,
                        content: Some(partial),
                        tool_calls: None,
                    },
                    finish_reason: None,
                }],
                usage: None,
            });
        }
    }

    chunks.push(ChatCompletionChunkResponse {
        id,
        object: "chat.completion.chunk".into(),
        created,
        model: "vibe-model".into(),
        choices: vec![ChoiceDeltaResponse {
            index: 0,
            delta: DeltaResponse {
                role: None,
                content: Some(String::new()),
                tool_calls: None,
            },
            finish_reason: Some("stop".into()),
        }],
        usage: None,
    });

    chunks
}
