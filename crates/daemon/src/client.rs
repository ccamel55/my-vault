use crate::{config, constants};

use shared_core::crypt;
use shared_core::database;
use std::sync::Arc;

/// Name of database file
const DATABASE_FILE_NAME: &str = "daemon.sqlite3";

/// Name of JWT RSA pem file
const RSA_PRIVATE_PEM_FILE_NAME: &str = "rsa.pem";

/// Holds information about current daemon client.
#[derive(Debug)]
pub struct DaemonClient {
    jwt: crypt::JwtFactory<Self>,
    time_start: chrono::DateTime<chrono::Utc>,
    database: database::Database,
}

impl crypt::JwtFactoryMetadata for DaemonClient {
    const ISSUER: &'static str = "my-vault-service";
}

impl DaemonClient {
    /// Create an instance of the client.
    pub async fn start(config: Arc<config::ConfigManager>) -> anyhow::Result<Self> {
        let config = config.config.read().await.database.clone();
        let database = database::Database::load(
            &constants::GLOBAL_CONFIG_PATH.join(DATABASE_FILE_NAME),
            config.encryption_key,
        )
        .await?;

        // Perform migration to ensure that our database is always upto date.
        sqlx::migrate!().run(database.get_pool()).await?;

        Ok(Self {
            jwt: crypt::JwtFactory::new(
                &constants::GLOBAL_CONFIG_PATH.join(RSA_PRIVATE_PEM_FILE_NAME),
            )
            .await?,
            time_start: chrono::Utc::now(),
            database,
        })
    }

    /// Get jwt factory instance.
    pub fn get_jwt_factory(&self) -> &crypt::JwtFactory<Self> {
        &self.jwt
    }

    /// Get time daemon was started.
    pub fn get_time_started(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.time_start
    }

    /// Get current database.
    pub fn get_database(&self) -> &database::Database {
        &self.database
    }
}
