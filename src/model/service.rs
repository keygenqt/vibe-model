//! Prompt processing service.
//! Sends messages to vibe-cluster and formats responses.

use crate::configs::app::AppConfig;
use crate::model::error::ModelError;
use crate::model::formats;
use crate::model::types::request::{ChatCompletionRequest, Content, Message};
use crate::model::types::response::{
    ChatCompletionChunk, ChatCompletionResponse, ChoiceDelta, Delta,
};
use vibe_cluster::{Cluster, ConnectionParams, Prompt, Provider};

/// Send prompt to cluster and return sync response.
pub async fn process(
    config: &AppConfig,
    request: &ChatCompletionRequest,
    user_agent: &str,
) -> Result<ChatCompletionResponse, ModelError> {
    log_request_fields(request);

    let raw_prompt = last_user_message(request);
    let format = formats::detect(user_agent);
    let prompt_text = format.clean_prompt(&raw_prompt);
    tracing::trace!("Full prompt:\n{}", prompt_text);

    // Check system questions about the model
    if let Some(canned) = check_system_question(&prompt_text) {
        tracing::trace!("System question matched, returning canned response");
        let message = format.format_message(&canned);
        return Ok(ChatCompletionResponse::from_message(
            &request.model,
            message,
        ));
    }

    let cluster = build_cluster(config).map_err(|e| ModelError::internal(&e))?;

    cluster
        .call(Prompt::new(prompt_text))
        .await
        .map(|result| {
            let content = result.text.unwrap_or_default();
            tracing::trace!("Full response:\n{}", content);
            let message = format.format_message(&content);
            ChatCompletionResponse::from_message(&request.model, message)
        })
        .map_err(|e| ModelError::internal(&e.to_string()))
}

/// Send prompt to cluster and return stream response.
pub async fn process_stream(
    config: &AppConfig,
    request: &ChatCompletionRequest,
    user_agent: &str,
) -> Result<Vec<ChatCompletionChunk>, ModelError> {
    log_request_fields(request);

    let raw_prompt = last_user_message(request);
    let format = formats::detect(user_agent);
    let prompt_text = format.clean_prompt(&raw_prompt);
    tracing::trace!("Full stream prompt:\n{}", prompt_text);

    // Check system questions about the model
    if let Some(canned) = check_system_question(&prompt_text) {
        tracing::trace!("System question matched, returning canned stream");
        let message = format.format_message(&canned);
        return Ok(build_stream_chunks(&request.model, &message));
    }

    let cluster = build_cluster(config).map_err(|e| ModelError::internal(&e))?;

    let result = cluster
        .call(Prompt::new(prompt_text))
        .await
        .map_err(|e| ModelError::internal(&e.to_string()))?;

    let content = result.text.unwrap_or_default();
    tracing::trace!("Full stream response:\n{}", content);
    let message = format.format_message(&content);
    Ok(build_stream_chunks(&request.model, &message))
}

/// Build stream chunks from message.
fn build_stream_chunks(model: &str, message: &Message) -> Vec<ChatCompletionChunk> {
    let id = format!("chatcmpl-{}", uuid::Uuid::new_v4());
    let created = now_secs();
    let mut chunks = Vec::new();

    // If message has tool_calls, return single chunk with tool_calls
    if let Some(tool_calls) = &message.tool_calls {
        chunks.push(ChatCompletionChunk {
            id: id.clone(),
            object: "chat.completion.chunk".into(),
            created,
            model: model.into(),
            choices: vec![ChoiceDelta {
                index: 0,
                delta: Delta {
                    role: Some("assistant".into()),
                    content: None,
                    tool_calls: Some(tool_calls.clone()),
                },
                finish_reason: Some("tool_calls".into()),
            }],
        });
        return chunks;
    }

    // Plain text streaming
    let content = content_to_string(&message.content);
    let words: Vec<&str> = content.split_whitespace().collect();

    if !words.is_empty() {
        chunks.push(ChatCompletionChunk {
            id: id.clone(),
            object: "chat.completion.chunk".into(),
            created,
            model: model.into(),
            choices: vec![ChoiceDelta {
                index: 0,
                delta: Delta {
                    role: Some("assistant".into()),
                    content: None,
                    tool_calls: None,
                },
                finish_reason: None,
            }],
        });

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
                model: model.into(),
                choices: vec![ChoiceDelta {
                    index: 0,
                    delta: Delta {
                        role: None,
                        content: Some(partial),
                        tool_calls: None,
                    },
                    finish_reason: None,
                }],
            });
        }
    }

    chunks.push(ChatCompletionChunk {
        id,
        object: "chat.completion.chunk".into(),
        created,
        model: model.into(),
        choices: vec![ChoiceDelta {
            index: 0,
            delta: Delta {
                role: None,
                content: Some(String::new()),
                tool_calls: None,
            },
            finish_reason: Some("stop".into()),
        }],
    });

    chunks
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
        .map(|m| content_to_string(&m.content))
        .unwrap_or_default()
}

/// Convert Content enum to plain string.
fn content_to_string(content: &Option<Content>) -> String {
    match content {
        Some(Content::Text(text)) => text.clone(),
        Some(Content::Parts(parts)) => parts
            .iter()
            .filter_map(|p| p.text.clone())
            .collect::<Vec<_>>()
            .join("\n"),
        None => String::new(),
    }
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

/// Check if prompt is a system question about the model itself.
fn check_system_question(prompt: &str) -> Option<String> {
    let prompt_lower = prompt.to_lowercase();
    let prompt_clean = prompt_lower
        .trim()
        .trim_end_matches(|c: char| c == '?' || c == '!' || c == '.')
        .trim();

    let system_patterns = [
        "что ты за модель",
        "какая ты модель",
        "кто ты",
        "как тебя зовут",
        "что ты такое",
        "what model are you",
        "who are you",
        "what is your name",
    ];

    if system_patterns
        .iter()
        .any(|p| prompt_clean.contains(&p.to_lowercase()))
    {
        Some(
            "Я — vibe-model, OpenAI-совместимое API, которое маршрутизирует запросы через кластер LLM. \
            Под капотом — vibe-cluster с поддержкой Ollama, DeepSeek, Qwen. \
            Выбираю лучшую модель под твою задачу."
                .into(),
        )
    } else {
        None
    }
}
