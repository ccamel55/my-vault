use crate::client::DaemonClient;
use crate::config::ConfigManager;
use shared_core::database;
use std::sync::Arc;

/// Secrets controller
#[derive(Debug, Clone)]
pub struct ControllerSecrets {
    pub(crate) config: Arc<ConfigManager>,
    pub(crate) client: Arc<DaemonClient>,
}

impl database::TableName for ControllerSecrets {
    const NAME: &'static str = "secrets";
}

#[cfg(test)]
mod tests {}
