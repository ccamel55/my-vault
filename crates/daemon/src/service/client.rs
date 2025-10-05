use crate::client::DaemonClient;

use shared_service::{InfoResponse, SetConnectionUrlsRequest, client_server};
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

    async fn set_connection_urls(
        &self,
        request: Request<SetConnectionUrlsRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();

        // TODO: validate input before updating config.

        // Update connection config
        {
            let mut config = self.client.get_config().connection.write().await;

            if let Some(url_api) = req.url_api {
                config.connection.url_api = url_api;
            }

            if let Some(url_identity) = req.url_identity {
                config.connection.url_identity = url_identity;
            }
        }

        // Try to write back changes to disk.
        // We GAF about whether this succeeds in the service routine.
        let _ = self.client.get_config().try_save().await;

        Ok(Response::new(()))
    }
}
