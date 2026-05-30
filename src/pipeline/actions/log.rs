//! Log action.
//! Logs current pipeline state for debugging.

use anyhow::Result;

use crate::pipeline::action::PipelineAction;
use crate::pipeline::context::PipelineContext;

/// Minimal log output.
const LOG_LEVEL: LogLevel = LogLevel::Cluster;

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum LogLevel {
    /// Skip data logging.
    Off,
    /// Log raw request data.
    Request,
    /// Log cleaned/parsed data.
    Parse,
    /// Log data sent to cluster.
    Cluster,
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

        log.push_str("\n╔═══════════════════════════\n");
        log.push_str("║ REQUEST DEBUG\n");
        log.push_str("╚═══════════════════════════\n\n");
        log.push_str(&format!("╠═ Model: {}\n", ctx.request.model));
        log.push_str(&format!("╠═ Stream: {:?}\n", ctx.request.stream));
        log.push_str(&format!("╠═ Messages: {}\n", ctx.request.messages.len()));
        log.push_str(&format!("╠═ Tools: {:?}\n", ctx.request.available_tools()));

        if LOG_LEVEL == LogLevel::Request {
            if let Some(system) = ctx.request.system_prompt() {
                log.push_str("\n╔═══════════════════════════\n");
                log.push_str(&format!(
                    "║ SYSTEM PROMPT ({} lines)\n",
                    system.lines().count()
                ));
                log.push_str("╚═══════════════════════════\n");
                log.push_str(&system);
            }

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

        if LOG_LEVEL == LogLevel::Parse {
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

        if LOG_LEVEL == LogLevel::Cluster {
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
                if let Some(usr) = &cluster.user_prompt {
                    log.push_str("\n╔═══════════════════════════\n");
                    log.push_str(&format!("║ CLUSTER USER ({}) lines\n", usr.lines().count()));
                    log.push_str("╚═══════════════════════════\n");
                    log.push_str(usr);
                }
                log.push_str("\n╔═══════════════════════════\n");
                log.push_str(&format!(
                    "║ CLUSTER RESPONSE ({} lines)\n",
                    cluster.response.lines().count()
                ));
                log.push_str("╚═══════════════════════════\n");
                log.push_str(&cluster.response);
            }
        }

        log.push_str("\n════════════════════════════");
        tracing::debug!("{}", log);
        Ok(())
    }
}
