mod login;

use crate::GlobalConfigs;

use clap::{Parser, Subcommand};
use std::sync::Arc;

/// Bitwarden CLI crab edition
#[derive(Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// URL to server
    #[clap(short, long, global = true)]
    pub server: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    /// Login to an existing account
    Login {
        #[command(subcommand)]
        login_type: login::LoginType,
    },
}

impl Commands {
    /// Run the given command.
    pub async fn run(
        &self,
        config: Arc<GlobalConfigs>,
        client: bitwarden_core::Client,
    ) -> anyhow::Result<()> {
        match self {
            Self::Login { login_type } => {
                login::run_login(config, client, login_type.to_owned()).await?
            }
        };

        Ok(())
    }
}
