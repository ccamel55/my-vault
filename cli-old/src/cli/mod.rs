mod login;
mod logout;

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
        /// Alias of existing user login
        alias: Option<String>,

        #[clap(long, action)]
        new: bool,
    },

    /// Log out of an existing frames
    Logout {
        /// Alias of existing user login
        alias: Option<String>,
    },
}

impl Commands {
    /// Run the given command.
    pub async fn run(
        self,
        config: Arc<GlobalConfigs>,
        client: bitwarden_core::Client,
    ) -> anyhow::Result<()> {
        match self {
            Self::Login { alias, new } => {
                if new {
                    login::run_login_new(config, client).await?;
                } else {
                    login::run_login_alias(config, client, alias).await?;
                }
            }
            Self::Logout { alias } => {
                logout::run_logout(config, client, alias).await?;
            }
        };

        Ok(())
    }
}
