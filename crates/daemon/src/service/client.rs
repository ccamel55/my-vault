use crate::client::DaemonClient;

use shared_service::{InfoRequest, InfoResponse, client_server};
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
    async fn info(&self, _request: Request<InfoRequest>) -> Result<Response<InfoResponse>, Status> {
        todo!()
    }
}
