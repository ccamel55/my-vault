mod authentication;

use shared_core::crypt;

pub use authentication::*;

#[derive(Debug, Default, Clone)]
pub struct RequestExtension {
    // If a JWT is valid then it will be passed through to the request
    pub jwt_claim_access: Option<crypt::JwtClaimAccess>,
}
