use shared_core::{config, rng};

/// Length of default database encryption key
const DATABASE_ENCRYPTION_KEY_DEFAULT_LENGTH: usize = 32;

pub type LocalConfig = config::LocalConfig<Config>;

/// Global config
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
}

impl config::ConfigMetadata for Config {
    const FILE_NAME: &'static str = "config.toml";
}

/// Database config
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DatabaseConfig {
    pub encryption_key: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            encryption_key: rng::random_string(DATABASE_ENCRYPTION_KEY_DEFAULT_LENGTH),
        }
    }
}
