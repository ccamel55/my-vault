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
    pub async fn start() -> anyhow::Result<Self> {
        // Setup configs and database.
        let config = config::ConfigsDaemon::load().await?;
        let database = database::Database::load().await?;

        Ok(Self {
            time_start: time::Instant::now(),
            config,
            database,
        })
    }

    /// Try to save any client changes.
    pub async fn try_save(&self) -> anyhow::Result<()> {
        self.config.try_save().await?;

        Ok(())
    }
}
