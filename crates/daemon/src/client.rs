use crate::{config, constants};

use shared_core::crypt;
use shared_core::database;
use std::sync::Arc;

#[cfg(test)]
use sqlx::sqlite;

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
    #[cfg(test)]
    pub async fn mocked(pool: sqlite::SqlitePool) -> anyhow::Result<Self> {
        /// Mocked RSA key which must never be used for real token authing
        const RSA_PEM_MOCK: &str = "-----BEGIN PRIVATE KEY-----
MIIBVAIBADANBgkqhkiG9w0BAQEFAASCAT4wggE6AgEAAkEAl6zz9vR4GZkePHFN
f81yAKtn2+a0X1B2nKyQWUcXopzF/x2awhu0wXMWV6kxRDHSg5BxBHnvaI09VmEO
A0kxiwIDAQABAkBLaJKWmi7H00ekF1THkJX4XT+ypb3RkYiXFnhh2qWWk4OmdwOV
tzA6aK76AJ+W4pYCYhNZk7OWmMV6NcDuelepAiEA31tNYNLLkXU08cw+GtrbvII1
GeuCVitoGuP2mggyJHUCIQCt18P8JIuHP4HpuQfPvi5czb6TDlIbuSOgHhYbyys9
/wIgFp6bdnvCi+ePxhEGFRgm+q9BC2/zUiCxOU/u0GiWE2UCIBGJSXDe8uBCzMUZ
8CrJoX2lF4tYD3pSc8CMKGjHVuZbAiEAoVHy/Z1AeX4LADMJBjVXAZ3L5ueBB2dP
HCC/me2tP9c=
-----END PRIVATE KEY-----";

        Ok(Self {
            jwt: crypt::JwtFactory::from_pem(RSA_PEM_MOCK)?,
            time_start: chrono::Utc::now(),
            database: pool.into(),
        })
    }

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
