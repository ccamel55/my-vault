use crate::client::DaemonClient;
use crate::config::ConfigManager;
use shared_core::database;
use std::sync::Arc;

/// Secrets controller
#[derive(Debug, Clone)]
pub struct ControllerSecret {
    pub(crate) config: Arc<ConfigManager>,
    pub(crate) client: Arc<DaemonClient>,
}

impl database::TableName for ControllerSecret {
    const NAME: &'static str = "secrets";
}

#[cfg(test)]
mod tests {}
