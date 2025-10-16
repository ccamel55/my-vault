use std::ops::Add;

lazy_static::lazy_static! {
    /// Default access token expiration time
    static ref DEFAULT_EXPIRATION_TIME_ACCESS_TOKEN: chrono::Duration = {
        chrono::Duration::hours(1)
    };

    /// Default refresh token expiration time
    static ref DEFAULT_EXPIRATION_TIME_REFRESH_TOKEN: chrono::Duration = {
        chrono::Duration::days(14)
    };
}

/// Claim for access tokens.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JwtClaimAccess {
    // (issuer): Issuer of the JWT
    pub iss: String,
    // (subject): Subject of the JWT (the user)
    pub sub: String,
    // (expiration time): Time after which the JWT expires
    pub exp: i64,

    // Preferred e-mail address
    pub email: String,
}

impl JwtClaimAccess {
    pub fn new(issuer: &str, user_id: &str, email: &str) -> Self {
        Self {
            iss: issuer.into(),
            sub: user_id.into(),
            exp: chrono::offset::Utc::now()
                .add(*DEFAULT_EXPIRATION_TIME_ACCESS_TOKEN)
                .timestamp(),
            email: email.into(),
        }
    }
}

/// Claim for refresh tokens.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JwtClaimRefresh {
    // (issuer): Issuer of the JWT
    pub iss: String,
    // (subject): Subject of the JWT (the user)
    pub sub: String,
    // (expiration time): Time after which the JWT expires
    pub exp: i64,
}

impl JwtClaimRefresh {
    pub fn new(issuer: &str, user_id: &str) -> Self {
        Self {
            iss: issuer.into(),
            sub: user_id.into(),
            exp: chrono::offset::Utc::now()
                .add(*DEFAULT_EXPIRATION_TIME_REFRESH_TOKEN)
                .timestamp(),
        }
    }
}
