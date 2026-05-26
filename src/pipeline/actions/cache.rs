//! Cache action for quick responses.
//! Returns cached responses for known prompts without calling the model.

use anyhow::Result;

use crate::pipeline::action::PipelineAction;
use crate::pipeline::context::PipelineContext;

/// Cache action result.
#[derive(Debug, Clone)]
pub struct CacheActionContext {
    /// Cached response text.
    pub response: String,
}

/// Returns cached responses for system questions.
pub struct CacheAction;

#[async_trait::async_trait]
impl PipelineAction for CacheAction {
    fn name(&self) -> &'static str {
        "Cache"
    }

    fn enabled(&self, _ctx: &PipelineContext) -> bool {
        true
    }

    async fn execute(&self, ctx: &mut PipelineContext) -> Result<()> {
        // Action
        if ctx.prompt_raw.to_lowercase().contains("что ты за модель") {
            ctx.cache_action = Some(CacheActionContext {
                response: "Я — vibe-model, OpenAI-совместимое API, которое маршрутизирует запросы через кластер LLM. \
                          Под капотом — vibe-cluster с поддержкой Ollama, DeepSeek, Qwen. \
                          Выбираю лучшую модель под твою задачу.".to_string(),
            });
        }

        // Action completed, returning to pipeline
        Ok(())
    }
}
