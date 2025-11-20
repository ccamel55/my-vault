mod authorization;
mod set_header;

use std::sync::Arc;

pub use authorization::*;
pub use set_header::*;

/// Data being passed to all middleware
#[derive(Debug, Clone)]
pub struct MiddlewareData {
    pub(crate) config: Arc<crate::ConfigManager>,
    pub(crate) client: Arc<crate::DaemonClient>,
}

impl MiddlewareData {
    pub fn new(config: Arc<crate::ConfigManager>, client: Arc<crate::DaemonClient>) -> Self {
        Self { config, client }
    }
}
