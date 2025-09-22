use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub type LocalUserConfig = crate::config::LocalConfig<UserConfig>;

/// User config
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct UserConfig {
    pub users: HashMap<String, UserEntryConfig>,
}

/// Per user specific settings
#[derive(Clone, Deserialize, Serialize)]
pub struct UserEntryConfig {
    /// User email
    pub email: String,
}

impl crate::config::ConfigMetadata for UserConfig {
    fn filename() -> &'static str {
        "users.toml"
    }

    fn relative_path() -> PathBuf {
        PathBuf::new().join(Self::filename())
    }
}
