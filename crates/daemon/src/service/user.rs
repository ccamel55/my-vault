use crate::{client, config, database};

use shared_service::{
    AddRequest, AddResponse, AuthRequest, AuthResponse, RefreshRequest, RefreshResponse,
    user_server,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};

#[derive(Debug, Clone)]
pub struct UserService {
    controller: database::controller::ControllerUser,
}

impl UserService {
    pub fn new(
        config: Arc<config::ConfigManager>,
        client: Arc<client::DaemonClient>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            controller: database::controller::ControllerUser::new(config, client),
        })
    }
}

#[tonic::async_trait]
impl user_server::User for UserService {
    async fn auth(&self, _request: Request<AuthRequest>) -> Result<Response<AuthResponse>, Status> {
        todo!()
    }

    async fn refresh(
        &self,
        request: Request<RefreshRequest>,
    ) -> Result<Response<RefreshResponse>, Status> {
        let req = request.into_inner();

        let token_auth = self.controller.refresh(req.token_refresh).await?;
        let res = RefreshResponse { token_auth };

        Ok(Response::new(res))
    }

    async fn add(&self, request: Request<AddRequest>) -> Result<Response<AddResponse>, Status> {
        let req = request.into_inner();

        // Process request
        let (token_auth, token_refresh) = self.controller.add(req.username, req.password).await?;

        let res = AddResponse {
            token_auth,
            token_refresh,
        };

        Ok(Response::new(res))
    }
}
