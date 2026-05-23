//! Server management commands.
//! Start, stop, and check vibe-model HTTP server status.

use clap::Subcommand;

/// Server management actions.
#[derive(Subcommand)]
pub enum ServeAction {
    /// Start vibe-model server.
    Start {
        /// Host address to bind (default: 127.0.0.1).
        #[arg(long)]
        host: Option<String>,

        /// Port to listen on (default: 3000).
        #[arg(long)]
        port: Option<u16>,
    },

    /// Stop running server(s).
    Stop {
        /// Stop only the server on this port. If not specified, stops all.
        #[arg(long)]
        port: Option<u16>,
    },

    /// Show server status.
    Status,
}

/// Execute serve commands.
pub async fn run(action: ServeAction) {
    match action {
        ServeAction::Start { host, port } => {
            // @todo: start server
            println!(
                "Starting server on {}:{}...",
                host.unwrap_or("127.0.0.1".into()),
                port.unwrap_or(3000)
            );
        }
        ServeAction::Stop { port } => {
            // @todo: stop server
            println!("Stopping server on port {:?}...", port);
        }
        ServeAction::Status => {
            // @todo: show server status
            println!("Server status: @todo");
        }
    }
}
