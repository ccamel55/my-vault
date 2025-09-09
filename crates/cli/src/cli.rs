use clap::{Parser, Subcommand};

/// Bitwarden CLI crab edition
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub sub_command: Option<SubCommands>,
}

#[derive(Subcommand, Debug)]
pub enum SubCommands {
    /// Login to a new account
    Login {
        /// Name of new account to save login as
        #[clap(long, required = true)]
        name: String,
    },

    /// Logout of an account
    Logout {
        /// Name of account to log out
        #[clap(long, required = true)]
        name: String,
    },

    /// Select account as current account
    Select {
        /// Name of account to select
        #[clap(long, required = true)]
        name: String,
    },
}
