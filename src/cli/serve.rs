//! Server management commands.
//! Start, stop, and check vibe-model HTTP server status.

use clap::Subcommand;

use crate::configs::app::AppConfig;
use crate::model::serve::ServeModel;
use crate::model::server;

/// Server management actions.
#[derive(Subcommand)]
pub enum ServeAction {
    /// Start vibe-model server.
    Start {
        /// Host address to bind.
        #[arg(long)]
        host: Option<String>,

        /// Port to listen on.
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
    let config = AppConfig::load().unwrap_or_default();

    match action {
        ServeAction::Start { host, port } => {
            let host = host.unwrap_or(config.server.host);
            let port = port.unwrap_or(config.server.port);

            let pid = std::process::id();
            let serve = ServeModel::new(pid, &host, port);
            serve.save().ok();

            if let Err(e) = server::run(host.clone(), port).await {
                serve.stop().ok();
                eprintln!("Failed to start server: {}", e);
            }
        }
        ServeAction::Stop { port } => {
            let servers = ServeModel::read().unwrap_or_default();
            for server in &servers {
                if port.is_none() || Some(server.port) == port {
                    server.stop().ok();
                }
            }
            println!("Server(s) stopped");
        }
        ServeAction::Status => match ServeModel::read() {
            Ok(servers) => {
                if servers.is_empty() {
                    println!("No servers running");
                } else {
                    println!("Servers running:");
                    for s in servers {
                        println!(
                            "  PID: {}  Host: {}  Port: {}  Uptime: {}",
                            s.pid,
                            s.host,
                            s.port,
                            s.load_time()
                        );
                    }
                }
            }
            Err(e) => eprintln!("Failed to read server stats: {}", e),
        },
    }
}
