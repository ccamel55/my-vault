use clap::{Subcommand, ValueEnum};

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
    /// Login to a user
    Login,

    /// Log out current user
    Logout,

    /// Lock current user
    Lock,

    /// Sync data between local and server
    Sync,

    /// List available objects of a type
    List { item_type: ListType, id: String },

    /// Get value for a given type
    Get { item_type: GetType },
}

impl Commands {
    /// Run the given command.
    pub async fn run(self) -> anyhow::Result<()> {
        match self {
            Self::Login => {}
            Self::Logout => {}
            Self::Lock => {}
            Self::Sync => {}
            Self::List { item_type, id } => {
                let _item_type = item_type;
                let _id = id;
            }
            Self::Get { item_type } => {
                let _item_type = item_type;
            }
        };

        Ok(())
    }
}
