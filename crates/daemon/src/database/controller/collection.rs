use crate::client::DaemonClient;
use crate::config::ConfigManager;
use shared_core::database;
use std::sync::Arc;

/// Collections controller
#[derive(Debug, Clone)]
pub struct ControllerCollections {
    pub(crate) config: Arc<ConfigManager>,
    pub(crate) client: Arc<DaemonClient>,
}

impl database::TableName for ControllerCollections {
    const NAME: &'static str = "collections";
}

#[cfg(test)]
mod tests {}
