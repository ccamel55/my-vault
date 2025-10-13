use crate::client::DaemonClient;

use shared_service::{
    LoginRequest, LoginResponse, RefreshRequest, RefreshResponse, RegisterRequest,
    RegisterResponse, auth_server,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};

#[derive(Debug, Clone)]
pub struct AuthService {
    client: Arc<DaemonClient>,
}

impl AuthService {
    pub fn new(client: Arc<DaemonClient>) -> anyhow::Result<Self> {
        Ok(Self { client })
    }
}

#[tonic::async_trait]
impl auth_server::Auth for AuthService {
    #[tracing::instrument]
    async fn login(
        &self,
        _request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        todo!()
    }

    #[tracing::instrument]
    async fn refresh(
        &self,
        _request: Request<RefreshRequest>,
    ) -> Result<Response<RefreshResponse>, Status> {
        todo!()
    }

    async fn register(
        &self,
        _request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        todo!()
    }
}
