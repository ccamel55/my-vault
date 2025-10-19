use crate::client;

use shared_core::crypt;
use std::sync::Arc;
use tonic::Status;
use tonic::body::Body;
use tonic::codegen::http::Request;

#[derive(Debug, Clone)]
pub struct Authentication {
    client: Arc<client::DaemonClient>,
}

impl Authentication {
    /// Create new authentication middleware instance.
    pub fn new(client: Arc<client::DaemonClient>) -> anyhow::Result<Self> {
        Ok(Self { client })
    }
}

#[tonic::async_trait]
impl tonic_middleware::RequestInterceptor for Authentication {
    async fn intercept(&self, mut req: Request<Body>) -> Result<Request<Body>, Status> {
        // Try check authorization
        let auth_jwt = req
            .headers()
            .get(tonic::codegen::http::header::AUTHORIZATION)
            .map(|x| x.to_str().unwrap());

        // Authorization must be provided
        let auth_jwt = match auth_jwt {
            Some(x) => x,
            None => {
                return Err(Status::unauthenticated("missing authorization credentials"));
            }
        };

        // Make sure JWT is valid
        let jwt_decode = self
            .client
            .get_jwt_factory()
            .decode::<crypt::JwtClaimAccess>(auth_jwt);

        match jwt_decode {
            Ok(x) => {
                tracing::debug!("{} -> valid auth jwt", req.uri());

                // Update JWT access claim
                req.extensions_mut()
                    .get_or_insert_default::<crate::middleware::RequestExtension>()
                    .jwt_claim_access = Some(x);
            }
            Err(e) => {
                tracing::debug!("{} -> invalid auth jwt: {}", req.uri(), e);
                return Err(Status::unauthenticated("invalid jwt"));
            }
        }

        Ok(req)
    }
}
