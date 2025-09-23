use clap::Subcommand;

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
    pub async fn run(self) -> anyhow::Result<()> {
        match self {
            Self::Login { alias, new } => {
                let _alias = alias;
                let _new = new;
            }
            Self::Logout { alias } => {
                let _alias = alias;
            }
        };

        Ok(())
    }
}
