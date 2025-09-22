use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub type LocalConnectionConfig = crate::config::LocalConfig<ClientConfig>;

/// Connection config
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct ClientConfig {
    pub connection: ConnectionConfig,
}

/// Client connection config
#[derive(Clone, Deserialize, Serialize)]
pub struct ConnectionConfig {
    /// Web endpoint for api server
    pub url_api: String,

    /// Web endpoint for identity server
    pub url_identity: String,
}

impl crate::config::ConfigMetadata for ClientConfig {
    fn filename() -> &'static str {
        "client.toml"
    }

    fn relative_path() -> PathBuf {
        PathBuf::new().join(Self::filename())
    }
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        // These defaults are for my self-hosted, please
        Self {
            url_api: "https://api.bitwarden.com".into(),
            url_identity: "https://identity.bitwarden.com".into(),
        }
    }
}
