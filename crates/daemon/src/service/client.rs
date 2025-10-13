use crate::client::DaemonClient;

use shared_service::{InfoResponse, client_server};
use std::sync::Arc;
use tonic::{Request, Response, Status};

#[derive(Debug)]
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
    async fn info(&self, _request: Request<()>) -> Result<Response<InfoResponse>, Status> {
        let time_start = *self.client.get_time_started();
        let config = self.client.get_config().connection.read().await;

        let res = InfoResponse {
            uptime_seconds: time_start.elapsed().as_secs(),
            url_api: config.connection.url_api.clone(),
            url_identity: config.connection.url_identity.clone(),
        };

        Ok(Response::new(res))
    }
}
