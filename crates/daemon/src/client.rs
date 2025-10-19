use crate::config;

use shared_core::crypt;
use shared_core::database;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Holds information about current daemon client.
#[derive(Debug)]
pub struct DaemonClient {
    jwt: crypt::JwtFactory<Self>,
    time_start: chrono::DateTime<chrono::Utc>,
    database: database::Database<Self>,
}

impl crypt::JwtFactoryMetadata for DaemonClient {
    const RSA_PEM_PRIVATE: &'static str = "rsa.pem";

    const ISSUER: &'static str = "my-vault-service";
}

impl database::DatabaseName for DaemonClient {
    const NAME: &'static str = "daemon.sqlite3";
}

impl DaemonClient {
    /// Create an instance of the client.
    pub async fn start(config: Arc<RwLock<config::LocalConfig>>) -> anyhow::Result<Self> {
        let config = config.read().await.database.clone();
        let database = database::Database::load(config.encryption_key).await?;

        // Perform migration to ensure that our database is always upto date.
        sqlx::migrate!().run(database.get_pool()).await?;

        Ok(Self {
            jwt: crypt::JwtFactory::new().await?,
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
    pub fn get_database(&self) -> &database::Database<Self> {
        &self.database
    }
}
