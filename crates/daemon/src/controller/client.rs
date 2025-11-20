use crate::client::DaemonClient;
use crate::error;

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

    /// Get elapsed time since startup
    pub fn uptime_seconds(&self) -> Result<u64, error::ServiceError> {
        let time_start = *self.client.get_time_started();
        let time_elapsed = chrono::Utc::now() - time_start;

        Ok(time_elapsed.num_seconds() as u64)
    }
}

#[cfg(test)]
mod tests {
    use crate::client;
    use crate::controller::ControllerClient;

    use sqlx::sqlite;
    use std::sync::Arc;

    #[sqlx::test]
    async fn uptime(pool: sqlite::SqlitePool) -> sqlx::Result<()> {
        let client = client::DaemonClient::mocked(pool)
            .await
            .expect("could not create mocked client");

        let controller = ControllerClient::new(Arc::new(client));

        let result = controller.uptime_seconds();

        assert!(result.is_ok());

        Ok(())
    }
}
