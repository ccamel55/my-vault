use std::path::PathBuf;
use thiserror::Error;
use tonic::transport::{Channel, Endpoint};

/// Client error
#[derive(Debug, Error)]
pub enum Error {
    #[error("connection error - {0}")]
    Connection(String),
}

/// Holds information about current cli client.
#[derive(Debug)]
pub struct CliClient {
    uds_path: PathBuf,
}

impl CliClient {
    /// Create new instance of CLI Client
    pub fn new(uds_path: PathBuf) -> Self {
        Self { uds_path }
    }

    /// Get channel connection, creating a connection if none.
    pub async fn channel(&self) -> Result<Channel, Error> {
        // - unix:relative_path
        // - unix:///absolute_path
        let uds_path = format!(
            "unix://{}",
            self.uds_path
                .canonicalize()
                .map_err(|e| Error::Connection(e.to_string()))?
                .display()
        );

        tracing::debug!("creating socket connection: {uds_path}");

        Endpoint::try_from(uds_path)
            .map_err(|e| Error::Connection(e.to_string()))?
            .connect()
            .await
            .map_err(|e| Error::Connection(e.to_string()))
    }
}
