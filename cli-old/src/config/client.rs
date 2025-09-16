use crate::config::{ConfigMetadata, LocalConfig};

use bitwarden_core::ClientSettings;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub type LocalClientConfig = LocalConfig<ClientConfig>;

/// Top level client settings
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

impl ConfigMetadata for ClientConfig {
    fn filename() -> &'static str {
        "client.toml"
    }

    fn relative_path() -> PathBuf {
        PathBuf::new().join(Self::filename())
    }
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        // Use same defaults as bitwarden client.
        let client_default = ClientSettings::default();

        Self {
            url_api: client_default.api_url,
            url_identity: client_default.identity_url,
        }
    }
}
