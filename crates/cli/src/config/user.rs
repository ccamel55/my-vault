use crate::config::{ConfigMetadata, LocalConfig};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub type LocalUserConfig = LocalConfig<UserConfig>;

/// Top level user settings
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct UserConfig {
    pub users: HashMap<String, UserEntryConfig>,
}

/// Per user specific settings
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct UserEntryConfig {
    /// Name for user
    pub alias: String,

    /// Email, password login only
    pub email: Option<String>,

    /// Client ID, api login only
    pub client_id: Option<String>,

    /// Client secret, api login only
    pub client_secret: Option<String>,
}

impl ConfigMetadata for UserConfig {
    fn filename() -> &'static str {
        "users.toml"
    }

    fn relative_path() -> PathBuf {
        PathBuf::new().join(Self::filename())
    }
}
