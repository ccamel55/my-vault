use crate::database;

use shared_core::{constants, crypt};

/// Holds information about current daemon client.
#[derive(Debug)]
pub struct DaemonClient {
    jwt: crypt::JwtFactory,
    time_start: chrono::DateTime<chrono::Utc>,
    database: database::Database,
}

impl DaemonClient {
    /// Create an instance of the client.
    pub async fn start(database: database::Database) -> anyhow::Result<Self> {
        Ok(Self {
            jwt: crypt::JwtFactory::new(constants::JWT_ISSUER).await?,
            time_start: chrono::Utc::now(),
            database,
        })
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
