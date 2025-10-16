use crate::client::DaemonClient;

use shared_service::{InfoResponse, client_server};
use std::sync::Arc;
use tonic::{Request, Response, Status};

#[derive(Debug, Clone)]
pub struct ClientService {
    client: Arc<DaemonClient>,
}

impl ClientService {
    pub fn new(client: Arc<DaemonClient>) -> anyhow::Result<Self> {
        Ok(Self { client })
    }
}

#[tonic::async_trait]
impl client_server::Client for ClientService {
    #[tracing::instrument]
    async fn info(&self, _request: Request<()>) -> Result<Response<InfoResponse>, Status> {
        let time_start = *self.client.get_time_started();
        let time_elapsed = chrono::Utc::now() - time_start;

        let res = InfoResponse {
            uptime_seconds: time_elapsed.num_seconds(),
        };

        Ok(Response::new(res))
    }
}
