use crate::client::DaemonClient;
use crate::config::ConfigManager;
use crate::database::view;

use shared_core::crypt::JwtFactoryMetadata;
use shared_core::{crypt, database, rng};
use std::str::FromStr;
use std::sync::Arc;

/// User controller
#[derive(Debug, Clone)]
pub struct ControllerUser {
    pub(crate) config: Arc<ConfigManager>,
    pub(crate) client: Arc<DaemonClient>,
}

impl database::TableName for ControllerUser {
    const NAME: &'static str = "users";
}

impl ControllerUser {
    pub fn new(config: Arc<ConfigManager>, client: Arc<DaemonClient>) -> Self {
        Self { config, client }
    }

    /// Checks if user with username exists
    pub async fn exists(&self, username: String) -> Result<bool, super::ControllerError> {
        // Filter by username
        let filter = vec![("username", username)];
        let result = database::exists::<Self>(self.client.get_database().get_pool(), filter)
            .await
            .map_err(|e| super::ControllerError::Unknown(e.to_string()))?;

        Ok(result)
    }

    pub async fn refresh(&self, token_refresh: String) -> Result<String, super::ControllerError> {
        let jwt_factory = self.client.get_jwt_factory();

        // Make sure that current token is valid
        let claim_refresh = jwt_factory
            .decode::<crypt::JwtClaimRefresh>(&token_refresh)
            .map_err(|_| {
                super::ControllerError::PermissionDenied("invalid refresh token".into())
            })?;

        // Fetch user based on uuid
        let uuid = uuid::Uuid::from_str(&claim_refresh.sub)
            .map_err(|e| super::ControllerError::Internal(e.to_string()))?;

        let filter = vec![("uuid", uuid.hyphenated().to_string())];
        let user =
            database::read::<Self, view::User>(self.client.get_database().get_pool(), filter)
                .await
                .map_err(|e| super::ControllerError::Unknown(e.to_string()))?;

        // Generate new auth token
        let token_auth = jwt_factory.encode(crypt::JwtClaimAccess::new(
            DaemonClient::ISSUER,
            user.uuid.into_uuid(),
            user.username,
        ));

        Ok(token_auth)
    }

    /// Add a new user
    pub async fn add(
        &self,
        username: String,
        password: String,
    ) -> Result<(String, String), super::ControllerError> {
        // Make sure that user doesn't exist.
        // If there is a user with the same identifier error as each username is assumed to be unique.
        if self.exists(username.clone()).await? {
            return Err(super::ControllerError::AlreadyExists(format!(
                "user with username {} already exists",
                &username
            )));
        }

        // Generate new user and password hash.
        let config = self.config.config.read().await.encryption.clone();

        let salt = rng::random_bytes_str(16);
        let password_hash = crypt::Argon2Factory::new(
            config.argon2_iters,
            config.argon2_memory_mb,
            config.argon2_parallelism,
        )
        .map_err(|e| super::ControllerError::Internal(e.to_string()))?
        .encode(password.as_bytes(), salt.as_bytes())
        .await
        .map_err(|e| super::ControllerError::Internal(e.to_string()))?;

        let data = view::User::new(
            username,
            password_hash,
            salt,
            config.argon2_iters,
            config.argon2_memory_mb,
            config.argon2_parallelism,
        )
        .map_err(|e| super::ControllerError::Internal(e.to_string()))?;

        // Try insert user into database and return auth tokens if successful.
        let user =
            database::create::<Self, view::User>(self.client.get_database().get_pool(), data)
                .await
                .map_err(|e| super::ControllerError::Unknown(e.to_string()))?;

        let jwt_factory = self.client.get_jwt_factory();

        let token_auth = jwt_factory.encode(crypt::JwtClaimAccess::new(
            DaemonClient::ISSUER,
            user.uuid.into_uuid(),
            user.username,
        ));

        let token_refresh = jwt_factory.encode(crypt::JwtClaimRefresh::new(
            DaemonClient::ISSUER,
            user.uuid.into_uuid(),
        ));

        Ok((token_auth, token_refresh))
    }
}
