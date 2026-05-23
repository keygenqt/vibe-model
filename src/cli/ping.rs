//! Ping command for testing cluster connectivity.
//! Sends test prompts and shows model status.

use clap::Subcommand;

/// Ping cluster for testing.
#[derive(Subcommand)]
pub enum PingAction {
    /// Send a test prompt and show response.
    Prompt {
        /// Prompt text.
        text: Vec<String>,
    },
    /// Show cluster status and available models.
    Status,
}

/// Execute ping commands.
pub async fn run(action: PingAction) {
    match action {
        PingAction::Prompt { text } => {
            let prompt = text.join(" ");
            println!("Prompt: {}", prompt);
            println!("Response: @todo");
        }
        PingAction::Status => {
            println!("Cluster status: @todo");
        }
    }
}
