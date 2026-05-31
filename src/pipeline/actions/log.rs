//! Log action.
//! Logs current pipeline state for debugging.

use anyhow::Result;

use crate::pipeline::action::PipelineAction;
use crate::pipeline::context::PipelineContext;

/// Log items output.
const LOG_ITEMS: &[LogItem] = &[LogItem::ComplexityResponse, LogItem::ClusterModels];

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum LogItem {
    /// Basic request info (model, stream, messages, tools).
    BaseInfo,
    /// Log raw system prompt.
    SystemPrompt,
    /// Log raw user prompt.
    UserPrompt,
    /// Log parsed system prompt.
    ParseSystem,
    /// Log parsed user prompt.
    ParseUser,
    /// Log cluster system prompt.
    ClusterSystemPrompt,
    /// Log cluster user prompt.
    ClusterUserPrompt,
    /// Log cluster response.
    ClusterResponse,
    /// Log models used for request.
    ClusterModels,
    /// Log complexity prompt sent to estimator.
    ComplexityPrompt,
    /// Log complexity response from estimator.
    ComplexityResponse,
}

/// Logs pipeline state for debugging.
pub struct LogAction;

#[async_trait::async_trait]
impl PipelineAction for LogAction {
    fn name(&self) -> &'static str {
        "Log"
    }

    fn enabled(&self, _ctx: &PipelineContext) -> bool {
        true
    }

    async fn execute(&self, ctx: &mut PipelineContext) -> Result<()> {
        let mut log = String::new();

        for item in LOG_ITEMS {
            // Basic request info.
            if item == &LogItem::BaseInfo {
                log.push_str("\n╔═══════════════════════════\n");
                log.push_str("║ REQUEST DEBUG\n");
                log.push_str("╚═══════════════════════════\n\n");
                log.push_str(&format!("╠═ Model: {}\n", ctx.request.model));
                log.push_str(&format!("╠═ Stream: {:?}\n", ctx.request.stream));
                log.push_str(&format!("╠═ Messages: {}\n", ctx.request.messages.len()));
                log.push_str(&format!("╠═ Tools: {:?}\n", ctx.request.available_tools()));
            }

            // Raw system prompt.
            if item == &LogItem::SystemPrompt {
                if let Some(system) = ctx.request.system_prompt() {
                    log.push_str("\n╔═══════════════════════════\n");
                    log.push_str(&format!(
                        "║ SYSTEM PROMPT ({} lines)\n",
                        system.lines().count()
                    ));
                    log.push_str("╚═══════════════════════════\n");
                    log.push_str(&system);
                }
            }

            // Raw user prompt.
            if item == &LogItem::UserPrompt {
                if let Some(prompt) = ctx.request.user_prompt() {
                    log.push_str("\n╔═══════════════════════════\n");
                    log.push_str(&format!(
                        "║ USER PROMPT ({} lines)\n",
                        prompt.lines().count()
                    ));
                    log.push_str("╚═══════════════════════════\n");
                    log.push_str(&prompt);
                }
            }

            // Parsed system prompt.
            if item == &LogItem::ParseSystem {
                if let Some(parse) = &ctx.parse_action {
                    if let Some(prompt) = parse.system.make_full_prompt() {
                        log.push_str("\n╔═══════════════════════════\n");
                        log.push_str(&format!(
                            "║ PARSE SYSTEM ({} lines)\n",
                            prompt.lines().count()
                        ));
                        log.push_str("╚═══════════════════════════\n");
                        log.push_str(&prompt);
                    }
                }
            }

            // Parsed user prompt.
            if item == &LogItem::ParseUser {
                if let Some(parse) = &ctx.parse_action {
                    let prompt = parse.user.make_prompt();
                    log.push_str("\n╔═══════════════════════════\n");
                    log.push_str(&format!(
                        "║ PARSE USER ({} lines)\n",
                        prompt.lines().count()
                    ));
                    log.push_str("╚═══════════════════════════\n");
                    log.push_str(&prompt);
                }
            }

            // Cluster system prompt.
            if item == &LogItem::ClusterSystemPrompt {
                if let Some(cluster) = &ctx.cluster_action {
                    if let Some(sys) = &cluster.system_prompt {
                        log.push_str("\n╔═══════════════════════════\n");
                        log.push_str(&format!(
                            "║ CLUSTER SYSTEM ({}) lines\n",
                            sys.lines().count()
                        ));
                        log.push_str("╚═══════════════════════════\n");
                        log.push_str(sys);
                    }
                }
            }

            // Cluster user prompt.
            if item == &LogItem::ClusterUserPrompt {
                if let Some(cluster) = &ctx.cluster_action {
                    if let Some(usr) = &cluster.user_prompt {
                        log.push_str("\n╔═══════════════════════════\n");
                        log.push_str(&format!("║ CLUSTER USER ({}) lines\n", usr.lines().count()));
                        log.push_str("╚═══════════════════════════\n");
                        log.push_str(usr);
                    }
                }
            }

            // Cluster response.
            if item == &LogItem::ClusterResponse {
                if let Some(cluster) = &ctx.cluster_action {
                    log.push_str("\n╔═══════════════════════════\n");
                    log.push_str(&format!(
                        "║ CLUSTER RESPONSE ({} lines)\n",
                        cluster.response.lines().count()
                    ));
                    log.push_str("╚═══════════════════════════\n");
                    log.push_str(&cluster.response);
                }
            }

            // Models used.
            if item == &LogItem::ClusterModels {
                if let Some(cluster) = &ctx.cluster_action {
                    if !cluster.models.is_empty() {
                        log.push_str("\n╔═══════════════════════════\n");
                        log.push_str("║ USING MODELS\n");
                        log.push_str("╚═══════════════════════════\n");
                        log.push_str(&cluster.models.join(", "));
                    }
                }
            }

            // Complexity prompt.
            if item == &LogItem::ComplexityPrompt {
                if let Some(complexity) = &ctx.complexity_action {
                    log.push_str("\n╔═══════════════════════════\n");
                    log.push_str("║ COMPLEXITY PROMPT\n");
                    log.push_str("╚═══════════════════════════\n");
                    log.push_str(&complexity.prompt);
                }
            }

            // Complexity response.
            if item == &LogItem::ComplexityResponse {
                if let Some(complexity) = &ctx.complexity_action {
                    log.push_str("\n╔═══════════════════════════\n");
                    log.push_str("║ COMPLEXITY RESPONSE\n");
                    log.push_str("╚═══════════════════════════\n");
                    log.push_str(&complexity.response);
                }
            }
        }

        if !log.is_empty() {
            log.push_str("\n════════════════════════════\n");
            tracing::debug!("{}", log);
        }

        Ok(())
    }
}
