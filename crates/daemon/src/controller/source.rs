use crate::client::DaemonClient;
use crate::config::ConfigManager;
use shared_core::database;
use std::sync::Arc;

/// Secrets controller
#[derive(Debug, Clone)]
pub struct ControllerSource {
    pub(crate) config: Arc<ConfigManager>,
    pub(crate) client: Arc<DaemonClient>,
}

impl database::TableName for ControllerSource {
    const NAME: &'static str = "sources";
}

#[cfg(test)]
mod tests {}
