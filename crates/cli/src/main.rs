mod client;
mod command;

use crate::client::CliClient;
use std::sync::Arc;

use clap::Parser;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

/// Bitwarden CLI crab edition
#[derive(Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: command::Commands,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    shared_core::tracing::init_subscriber(shared_core::Client::Cli)?;

    let task_tracker = TaskTracker::new();
    let cancellation_token = CancellationToken::new();

    // Install signal handler to listen for cancellation.
    let signal_handle = shared_core::signal::listen_for_cancellation(
        task_tracker.clone(),
        cancellation_token.clone(),
    )?;

    let args = Cli::parse();
    let client = Arc::new(CliClient::new(shared_core::local_socket_path()));

    // Execute what ever command is being passed.
    args.command
        .run(
            task_tracker.clone(),
            cancellation_token.clone(),
            client.clone(),
        )
        .await?;

    // Close signal stream
    signal_handle.close();

    // Wait for everything to finish before exiting
    task_tracker.close();
    task_tracker.wait().await;

    Ok(())
}
