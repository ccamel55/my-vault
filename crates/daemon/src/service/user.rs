use shared_service::{LoginRequest, LoginResponse, LogoutRequest, LogoutResponse, user_server};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct UserService;

#[tonic::async_trait]
impl user_server::User for UserService {
    async fn login(
        &self,
        _request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        todo!()
    }

    async fn logout(
        &self,
        _request: Request<LogoutRequest>,
    ) -> Result<Response<LogoutResponse>, Status> {
        todo!()
    }
}
