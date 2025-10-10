use crate::error::Error;

use std::path::PathBuf;
use tonic::transport::{Channel, Endpoint};

/// Holds information about current cli client.
#[derive(Debug)]
pub struct CliClient {
    uds_path: PathBuf,
    terminal: console::Term,
}

impl CliClient {
    /// Create new instance of CLI Client
    pub fn new(uds_path: PathBuf) -> Self {
        Self {
            uds_path,
            terminal: console::Term::stdout(),
        }
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

    /// Get terminal handle.
    pub fn terminal(&self) -> &console::Term {
        &self.terminal
    }
}
