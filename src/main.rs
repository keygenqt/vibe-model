//! vibe-model entry point.
//! Parses CLI arguments and dispatches to server commands.

use std::path::PathBuf;

use clap::CommandFactory;
use clap::Parser;
use clap::Subcommand;

use crate::cli::ping::PingAction;
use crate::cli::serve::ServeAction;
use crate::configs::app::AppConfig;

mod cli;
mod clients;
mod configs;
mod models;
mod pipeline;
mod providers;
mod utils;

#[derive(Parser)]
#[command(name = utils::app::app_name())]
#[command(about = utils::app::app_about())]
#[command(styles = utils::app::app_styles())]
#[command(version = utils::app::app_version())]
struct App {
    /// Path to config file (default: ~/.vibe-model/config.json5).
    #[arg(long, global = true)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage vibe-model server.
    Serve {
        #[command(subcommand)]
        action: ServeAction,
    },

    /// Test cluster connectivity and models.
    Ping {
        #[command(subcommand)]
        action: PingAction,
    },
}

#[tokio::main]
async fn main() {
    let app = App::parse();

    // Load config with override
    AppConfig::set_global_config(app.config.clone());

    // Validate config
    if let Err(error) = AppConfig::validate() {
        exit_error!("{}", error);
    }

    match app.command {
        Some(Commands::Serve { action }) => cli::serve::run(action).await,
        Some(Commands::Ping { action }) => cli::ping::run(action).await,
        None => {
            let _ = App::command().print_help();
        }
    }
}
