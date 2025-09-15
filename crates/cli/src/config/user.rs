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
#[derive(Clone, Deserialize, Serialize)]
pub struct UserEntryConfig {
    /// User id
    pub user_id: Option<bitwarden_core::UserId>,

    /// User email
    pub email: String,

    /// User KDF parameters
    pub kdf: bitwarden_crypto::Kdf,

    /// Base64 encoded device key
    pub device_key: bitwarden_encoding::B64,

    /// UserKey encrypted with DevicePublicKey
    pub user_key: bitwarden_crypto::UnsignedSharedKey,

    /// DevicePublicKey encrypted with [UserKey](super::UserKey)
    pub device_public_key: bitwarden_crypto::EncString,

    /// DevicePrivateKey encrypted with [DeviceKey]
    pub device_private_key: bitwarden_crypto::EncString,
}

impl ConfigMetadata for UserConfig {
    fn filename() -> &'static str {
        "users.toml"
    }

    fn relative_path() -> PathBuf {
        PathBuf::new().join(Self::filename())
    }
}
