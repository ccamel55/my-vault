mod connection;
mod user;

use tokio::sync::RwLock;

pub use connection::LocalConnectionConfig;
pub use user::LocalUserConfig;

/// Daemon Configs
pub struct ConfigsDaemon {
    /// Connection config
    pub connection: RwLock<LocalConnectionConfig>,

    /// User config
    pub user: RwLock<LocalUserConfig>,
}

impl ConfigsDaemon {
    /// Create a new daemon config instance.
    pub async fn load() -> anyhow::Result<Self> {
        let global_config_path = shared_core::GLOBAL_CONFIG_PATH.to_path_buf();
        let global_config_path_exists = global_config_path.exists() && global_config_path.is_dir();

        tracing::debug!("config folder: {}", global_config_path.display());
        tracing::debug!("creating config folder: {}", !global_config_path_exists);

        // Make sure that our config folder exists
        if !global_config_path_exists {
            tokio::fs::create_dir(&global_config_path).await?;
        }

        let connection = LocalConnectionConfig::load(&global_config_path, true)
            .await
            .map(RwLock::new)?;

        let user = LocalUserConfig::load(&global_config_path, true)
            .await
            .map(RwLock::new)?;

        let result = Self { connection, user };

        Ok(result)
    }

    /// Attempt to do a save using `try_read`.
    pub async fn try_save(&self) -> anyhow::Result<()> {
        if let Ok(config) = self.connection.try_read() {
            config
                .save(shared_core::GLOBAL_CONFIG_PATH.as_path())
                .await?;
        }

        if let Ok(config) = self.user.try_read() {
            config
                .save(shared_core::GLOBAL_CONFIG_PATH.as_path())
                .await?;
        }

        Ok(())
    }
}
