use crate::client::DaemonClient;

use shared_service::{LoginRequest, LoginResponse, LogoutRequest, LogoutResponse, user_server};
use std::sync::Arc;
use tonic::{Request, Response, Status};

#[derive(Debug, Clone)]
pub struct UserService {
    client: Arc<DaemonClient>,
}

impl UserService {
    pub fn new(client: Arc<DaemonClient>) -> anyhow::Result<Self> {
        Ok(Self { client })
    }
}

#[tonic::async_trait]
impl user_server::User for UserService {
    #[tracing::instrument]
    async fn login(
        &self,
        _request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        todo!()
    }

    #[tracing::instrument]
    async fn logout(
        &self,
        _request: Request<LogoutRequest>,
    ) -> Result<Response<LogoutResponse>, Status> {
        todo!()
    }
}
