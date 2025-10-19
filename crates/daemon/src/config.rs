use crate::constants;
use shared_core::{config, rng};
use tokio::sync::RwLock;

/// Name of config file
const CONFIG_FILE_NAME: &str = "config.toml";

/// Length of default database encryption key
const DATABASE_ENCRYPTION_KEY_DEFAULT_LENGTH: usize = 32;

/// Global config
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
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

// New type wrappers for config because i'm too lazy to implement derive macros.
// TODO ALLAN: create derive macros for implementing traits.
pub type LocalConfig = config::LocalConfig<Config>;

/// Local config handler
pub struct ConfigManager {
    /// Main config file
    pub config: RwLock<LocalConfig>,
}

impl ConfigManager {
    pub async fn load() -> anyhow::Result<Self> {
        let config = LocalConfig::load(
            constants::GLOBAL_CONFIG_PATH
                .join(CONFIG_FILE_NAME)
                .as_path(),
        )
        .await?;

        Ok(Self {
            config: RwLock::new(config),
        })
    }

    pub async fn save(&self) -> anyhow::Result<()> {
        self.config
            .write()
            .await
            .save(
                constants::GLOBAL_CONFIG_PATH
                    .join(CONFIG_FILE_NAME)
                    .as_path(),
            )
            .await?;

        Ok(())
    }
}
