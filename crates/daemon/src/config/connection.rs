use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub type LocalConnectionConfig = shared_core::config::LocalConfig<ClientConfig>;

/// Connection config
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ClientConfig {
    pub connection: ConnectionConfig,
    pub default: ConnectionDefaultConfig,
}

/// Client connection config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConnectionConfig {
    /// Web endpoint for api server
    pub url_api: String,

    /// Web endpoint for identity server
    pub url_identity: String,
}

/// Client defaults config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConnectionDefaultConfig {
    /// Time before automatically locking data in seconds.
    pub time_to_lock: u64,
}

impl shared_core::config::ConfigMetadata for ClientConfig {
    fn filename() -> &'static str {
        "client.toml"
    }

    fn relative_path() -> PathBuf {
        PathBuf::new().join(Self::filename())
    }
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            url_api: "https://api.bitwarden.com".into(),
            url_identity: "https://identity.bitwarden.com".into(),
        }
    }
}

impl Default for ConnectionDefaultConfig {
    fn default() -> Self {
        Self { time_to_lock: 300 }
    }
}
