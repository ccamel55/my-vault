mod config;
mod info;

use crate::client::CliClient;
use crate::error::Error;

use clap::{Subcommand, ValueEnum};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

#[derive(Clone, Debug, ValueEnum)]
pub enum ListType {
    Item,
    Folder,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum GetType {
    Username,
    Password,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    /// Info about daemon
    Info,

    /// Modify configuration
    Config(config::Args),
}

impl Commands {
    /// Run the given command.
    pub async fn run(
        &self,
        _task_tracker: TaskTracker,
        _cancellation_token: CancellationToken,
        client: Arc<CliClient>,
    ) -> Result<(), Error> {
        match self {
            Self::Info => info::run_cmd(client).await?,
            Self::Config(args) => config::run_cmd(client, args).await?,
        };

        Ok(())
    }
}
