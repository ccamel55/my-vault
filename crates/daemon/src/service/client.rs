use shared_service::{InfoRequest, InfoResponse, client_server};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct ClientService;

#[tonic::async_trait]
impl client_server::Client for ClientService {
    async fn info(&self, _request: Request<InfoRequest>) -> Result<Response<InfoResponse>, Status> {
        todo!()
    }
}
