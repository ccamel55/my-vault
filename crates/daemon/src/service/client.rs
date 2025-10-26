use crate::database;

use shared_service::{InfoResponse, client_server};
use tonic::{Request, Response, Status};

#[derive(Debug, Clone)]
pub struct ClientService {
    controller_client: database::controller::ControllerClient,
}

impl ClientService {
    pub fn new(controller_client: database::controller::ControllerClient) -> anyhow::Result<Self> {
        Ok(Self {
            controller_client: controller_client.clone(),
        })
    }
}

#[tonic::async_trait]
impl client_server::Client for ClientService {
    #[tracing::instrument]
    async fn info(&self, _request: Request<()>) -> Result<Response<InfoResponse>, Status> {
        let time_start = *self.controller_client.client.get_time_started();
        let time_elapsed = chrono::Utc::now() - time_start;

        let res = InfoResponse {
            uptime_seconds: time_elapsed.num_seconds(),
        };

        Ok(Response::new(res))
    }
}
