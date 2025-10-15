lazy_static::lazy_static! {
    /// Default access token expiration time
    static ref DEFAULT_EXPIRATION_TIME_ACCESS_TOKEN: std::time::Duration = {
        // 1 hour
        std::time::Duration::from_secs(60 * 60)
    };

    /// Default refresh token expiration time
    static ref DEFAULT_EXPIRATION_TIME_REFRESH_TOKEN: std::time::Duration = {
        // 14 days
        std::time::Duration::from_secs(60 * 24 * 14)
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
    pub exp: u64,

    // Preferred e-mail address
    pub email: String,
}

impl JwtClaimAccess {
    pub fn new(issuer: &str, user_id: &str, email: &str) -> Self {
        Self {
            iss: issuer.into(),
            sub: user_id.into(),
            exp: crate::time::utc_now()
                .checked_add(*DEFAULT_EXPIRATION_TIME_ACCESS_TOKEN)
                .expect("utc expiration time integer overflow")
                .as_secs(),
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
    pub exp: u64,
}

impl JwtClaimRefresh {
    pub fn new(issuer: &str, user_id: &str) -> Self {
        Self {
            iss: issuer.into(),
            sub: user_id.into(),
            exp: crate::time::utc_now()
                .checked_add(*DEFAULT_EXPIRATION_TIME_REFRESH_TOKEN)
                .expect("utc expiration time integer overflow")
                .as_secs(),
        }
    }
}
