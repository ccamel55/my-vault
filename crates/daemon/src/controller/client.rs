use crate::client::DaemonClient;

use std::sync::Arc;

/// Client controller
#[derive(Debug, Clone)]
pub struct ControllerClient {
    pub(crate) client: Arc<DaemonClient>,
}

impl ControllerClient {
    pub fn new(client: Arc<DaemonClient>) -> Self {
        Self { client }
    }
}
