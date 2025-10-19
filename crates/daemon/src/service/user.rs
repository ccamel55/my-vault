use crate::{client, config, database};

use shared_core::crypt;
use shared_core::crypt::JwtFactoryMetadata;
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
        _request: Request<RefreshRequest>,
    ) -> Result<Response<RefreshResponse>, Status> {
        todo!()
    }

    async fn add(&self, request: Request<AddRequest>) -> Result<Response<AddResponse>, Status> {
        let req = request.into_inner();

        // Make sure that user doesn't exist
        self.controller
            .exists(req.email.clone())
            .await
            .map_err(|_| Status::already_exists("user with email already exists"))?;

        // Add user to our database
        let entry = self
            .controller
            .add(req.email, req.password, req.first_name, req.last_name)
            .await
            .map_err(|e| Status::unknown(e.to_string()))?;

        // Generate JWT for user
        let jwt_factory = self.controller.client.get_jwt_factory();

        let token_auth = jwt_factory.encode(crypt::JwtClaimAccess::new(
            client::DaemonClient::ISSUER,
            entry.uuid.into_uuid(),
            entry.email,
        ));

        let token_refresh = jwt_factory.encode(crypt::JwtClaimRefresh::new(
            client::DaemonClient::ISSUER,
            entry.uuid.into_uuid(),
        ));

        let res = AddResponse {
            token_auth,
            token_refresh,
        };

        Ok(Response::new(res))
    }
}
