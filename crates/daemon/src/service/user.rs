use crate::controller;

use shared_service::{
    AddRequest, AddResponse, AuthRequest, AuthResponse, RefreshRequest, RefreshResponse,
    user_server,
};
use tonic::{Request, Response, Status};

#[derive(Debug, Clone)]
pub struct UserService {
    controller_user: controller::ControllerUser,
}

impl UserService {
    pub fn new(controller_user: controller::ControllerUser) -> anyhow::Result<Self> {
        Ok(Self {
            controller_user: controller_user.clone(),
        })
    }
}

#[tonic::async_trait]
impl user_server::User for UserService {
    async fn auth(&self, request: Request<AuthRequest>) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();

        // Process request
        let (token_auth, token_refresh) = self
            .controller_user
            .auth(req.username, req.password)
            .await?;

        let res = AuthResponse {
            token_auth,
            token_refresh,
        };

        Ok(Response::new(res))
    }

    async fn refresh(
        &self,
        request: Request<RefreshRequest>,
    ) -> Result<Response<RefreshResponse>, Status> {
        let req = request.into_inner();

        let token_auth = self.controller_user.refresh(req.token_refresh).await?;
        let res = RefreshResponse { token_auth };

        Ok(Response::new(res))
    }

    async fn add(&self, request: Request<AddRequest>) -> Result<Response<AddResponse>, Status> {
        let req = request.into_inner();

        // Process request
        let (token_auth, token_refresh) =
            self.controller_user.add(req.username, req.password).await?;

        let res = AddResponse {
            token_auth,
            token_refresh,
        };

        Ok(Response::new(res))
    }
}
