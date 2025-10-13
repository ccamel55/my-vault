use crate::database;
use tokio::time;

/// Holds information about current daemon client.
#[derive(Debug)]
pub struct DaemonClient {
    time_start: time::Instant,
    database: database::Database,
}

impl DaemonClient {
    /// Create an instance of the client.
    pub async fn start(database: database::Database) -> Self {
        Self {
            time_start: time::Instant::now(),
            database,
        }
    }

    /// Get time daemon was started.
    pub fn get_time_started(&self) -> &time::Instant {
        &self.time_start
    }

    /// Get current datbase.
    pub fn get_database(&self) -> &database::Database {
        &self.database
    }
}
