use crate::client::DaemonClient;
use crate::config::ConfigManager;
use crate::database::view;

use shared_core::{crypt, database, rng};
use std::sync::Arc;

/// User controller
#[derive(Debug, Clone)]
pub struct ControllerUser {
    pub(crate) config: Arc<ConfigManager>,
    pub(crate) client: Arc<DaemonClient>,
}

impl database::TableName for ControllerUser {
    const NAME: &'static str = "users";
}

impl ControllerUser {
    pub fn new(config: Arc<ConfigManager>, client: Arc<DaemonClient>) -> Self {
        Self { config, client }
    }

    /// Checks if user with email exists
    pub async fn exists(&self, email: String) -> anyhow::Result<bool> {
        // Filter by email
        let filter = vec![("email", email)];
        let result =
            database::exists::<Self>(self.client.get_database().get_pool(), filter).await?;

        Ok(result)
    }

    /// Add a new user
    pub async fn add(
        &self,
        email: String,
        password: String,
        first_name: String,
        last_name: String,
    ) -> anyhow::Result<view::User> {
        let config = self.config.config.read().await.encryption.clone();

        let salt = rng::random_bytes(64);
        let password_hash = crypt::Argon2Factory::new(
            config.argon2_iters,
            config.argon2_memory_mb,
            config.argon2_parallelism,
        )
        .map_err(|e| anyhow::anyhow!(e))?
        .encode(password.as_bytes(), &salt)
        .await?;

        let data = view::User::new(
            email,
            password_hash,
            first_name,
            last_name,
            salt,
            config.argon2_iters,
            config.argon2_memory_mb,
            config.argon2_parallelism,
        )?;

        let result =
            database::create::<Self, _>(self.client.get_database().get_pool(), data).await?;

        Ok(result)
    }
}
