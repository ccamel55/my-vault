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
    pub encryption: EncryptionConfig,
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

/// Encryption config
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EncryptionConfig {
    pub argon2_iters: u32,
    pub argon2_memory_mb: u32,
    pub argon2_parallelism: u32,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        // Values are chosen based of guidelines in the link bellow
        // https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#pre-hashing-passwords
        Self {
            argon2_iters: 2,
            argon2_memory_mb: 32,
            argon2_parallelism: 2,
        }
    }
}

// New type wrappers for config because i'm too lazy to implement derive macros.
// TODO ALLAN: create derive macros for implementing traits.
pub type LocalConfig = config::LocalConfig<Config>;

/// Local config handler
#[derive(Debug)]
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
