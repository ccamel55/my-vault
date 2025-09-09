use crate::cli::SubCommands;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::{EnvFilter, fmt};

mod cli;
mod constants;
mod tui;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Cli::parse();

    // TODO
    // - config system
    // - write logs to file
    // - setup bitwarden SDK crate
    // - implement basic TUI

    // Setup tracing subscriber
    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to set global tracing subscriber");

    if let Some(sub_command) = args.sub_command {
        match sub_command {
            SubCommands::Login { name } => Ok(()),
            SubCommands::Logout { name } => Ok(()),
            SubCommands::Select { name } => Ok(()),
        }
    } else {
        // Create TUI
        let mut tui = tui::Tui::start(constants::DEFAULT_FPS, constants::DEFAULT_TPS)?;

        // Shutdown TUI
        tui.wait().await
    }
}
