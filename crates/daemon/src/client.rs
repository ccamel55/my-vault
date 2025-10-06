use crate::{config, database};
use tokio::time;

/// Holds information about current daemon client.
#[derive(Debug)]
pub struct DaemonClient {
    time_start: time::Instant,
    config: config::ConfigsDaemon,
    database: database::Database,
}

impl DaemonClient {
    /// Create an instance of the client.
    pub async fn start(config: config::ConfigsDaemon, database: database::Database) -> Self {
        Self {
            time_start: time::Instant::now(),
            config,
            database,
        }
    }

    /// Try to save any client changes.
    pub async fn try_save(&self) -> anyhow::Result<()> {
        self.config.try_save().await?;

        Ok(())
    }

    /// Get time daemon was started.
    pub fn get_time_started(&self) -> &time::Instant {
        &self.time_start
    }

    /// Get current config.
    pub fn get_config(&self) -> &config::ConfigsDaemon {
        &self.config
    }

    /// Get current datbase.
    pub fn get_database(&self) -> &database::Database {
        &self.database
    }
}
