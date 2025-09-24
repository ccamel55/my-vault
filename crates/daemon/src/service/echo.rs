use shared_service::{EchoRequest, EchoResponse, echo_server};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct EchoService;

#[tonic::async_trait]
impl echo_server::Echo for EchoService {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let req = request.into_inner();
        let resp = EchoResponse {
            message: req.message,
        };

        Ok(Response::new(resp))
    }
}
