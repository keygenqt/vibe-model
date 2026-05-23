//! Prompt processing service.
//! Sends messages to vibe-cluster and formats responses.

use crate::configs::app::AppConfig;
use crate::model::error::ModelError;
use crate::model::types::request::ChatCompletionRequest;
use crate::model::types::request::Message;
use crate::model::types::response::{
    ChatCompletionChunk, ChatCompletionResponse, Choice, ChoiceDelta, Delta, Usage,
};
use vibe_cluster::{Cluster, ConnectionParams, Prompt, Provider};

/// Send prompt to cluster and return sync response.
pub async fn process(
    config: &AppConfig,
    request: &ChatCompletionRequest,
) -> Result<ChatCompletionResponse, ModelError> {
    log_request_fields(request);

    let cluster = build_cluster(config).map_err(|e| ModelError::internal(&e))?;

    let prompt_text = last_user_message(request);

    cluster
        .call(Prompt::new(prompt_text))
        .await
        .map(|result| {
            let content = result.text.unwrap_or_default();
            ChatCompletionResponse {
                id: format!("chatcmpl-{}", uuid::Uuid::new_v4()),
                object: "chat.completion".into(),
                created: now_secs(),
                model: request.model.clone(),
                choices: vec![Choice {
                    index: 0,
                    message: Message {
                        role: "assistant".into(),
                        content: Some(content),
                        tool_calls: None,
                        tool_call_id: None,
                    },
                    finish_reason: "stop".into(),
                }],
                usage: Usage {
                    prompt_tokens: 0,
                    completion_tokens: 0,
                    total_tokens: 0,
                },
            }
        })
        .map_err(|e| ModelError::internal(&e.to_string()))
}

/// Send prompt to cluster and return stream response.
pub async fn process_stream(
    config: &AppConfig,
    request: &ChatCompletionRequest,
) -> Result<Vec<ChatCompletionChunk>, ModelError> {
    log_request_fields(request);

    let cluster = build_cluster(config).map_err(|e| ModelError::internal(&e))?;

    let prompt_text = last_user_message(request);

    let result = cluster
        .call(Prompt::new(prompt_text))
        .await
        .map_err(|e| ModelError::internal(&e.to_string()))?;

    let content = result.text.unwrap_or_default();
    let id = format!("chatcmpl-{}", uuid::Uuid::new_v4());
    let created = now_secs();
    let model = request.model.clone();

    // Split content into words for fake streaming
    let words: Vec<&str> = content.split_whitespace().collect();
    let mut chunks = Vec::new();

    // First chunk with role
    if !words.is_empty() {
        chunks.push(ChatCompletionChunk {
            id: id.clone(),
            object: "chat.completion.chunk".into(),
            created,
            model: model.clone(),
            choices: vec![ChoiceDelta {
                index: 0,
                delta: Delta {
                    role: Some("assistant".into()),
                    content: None,
                },
                finish_reason: None,
            }],
        });

        // Content chunks — each word as delta
        for (i, word) in words.iter().enumerate() {
            let partial = if i == 0 {
                word.to_string()
            } else {
                format!(" {}", word)
            };
            chunks.push(ChatCompletionChunk {
                id: id.clone(),
                object: "chat.completion.chunk".into(),
                created,
                model: model.clone(),
                choices: vec![ChoiceDelta {
                    index: 0,
                    delta: Delta {
                        role: None,
                        content: Some(partial),
                    },
                    finish_reason: None,
                }],
            });
        }
    }

    // Final chunk with finish_reason
    chunks.push(ChatCompletionChunk {
        id: id.clone(),
        object: "chat.completion.chunk".into(),
        created,
        model: model.clone(),
        choices: vec![ChoiceDelta {
            index: 0,
            delta: Delta {
                role: None,
                content: Some(String::new()),
            },
            finish_reason: Some("stop".into()),
        }],
    });

    Ok(chunks)
}

/// Log request fields for debugging.
fn log_request_fields(request: &ChatCompletionRequest) {
    if let Some(tools) = &request.tools {
        for tool in tools {
            tracing::debug!(
                "Tool requested: {} — {}",
                tool.function.name,
                tool.function
                    .description
                    .as_deref()
                    .unwrap_or("no description")
            );
            if let Some(params) = &tool.function.parameters {
                tracing::debug!("  Parameters: {}", params);
            }
        }
    }
    if request.tool_choice.is_some() {
        tracing::debug!("Tool choice: {:?}", request.tool_choice);
    }
    if let Some(temp) = request.temperature {
        tracing::debug!("Requested temperature: {}, using cluster defaults", temp);
    }
    if let Some(max_tok) = request.max_tokens {
        tracing::debug!("Requested max_tokens: {}, using cluster defaults", max_tok);
    }
}

/// Extract last user message text.
fn last_user_message(request: &ChatCompletionRequest) -> String {
    request
        .messages
        .iter()
        .filter(|m| m.role == "user")
        .last()
        .map(|m| m.content.clone().unwrap_or_default())
        .unwrap_or_default()
}

/// Current unix timestamp in seconds.
fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Build vibe-cluster from config.
fn build_cluster(config: &AppConfig) -> Result<Cluster, String> {
    let connections: Vec<ConnectionParams> = config
        .cluster
        .iter()
        .map(|c| {
            let provider = match c.provider.as_str() {
                "ollama" => Provider::Ollama,
                "deepseek" => Provider::DeepSeek,
                "qwen" => Provider::Qwen,
                other => return Err(format!("Unknown provider: {}", other)),
            };
            Ok(ConnectionParams {
                provider,
                host: c.host.clone(),
                model: c.model.clone(),
                temperature: Some(c.temperature),
                seed: Some(c.seed),
                num_ctx: Some(c.num_ctx),
                num_predict: Some(c.num_predict),
                timeout_secs: Some(c.timeout_secs),
                api_key: c.api_key.clone(),
                parallel: c.parallel,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;

    Cluster::new(connections).map_err(|e| e.to_string())
}
