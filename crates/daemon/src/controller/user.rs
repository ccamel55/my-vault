use crate::client::DaemonClient;
use crate::config::ConfigManager;
use crate::error;
use crate::schema;

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
    const NAME: &'static str = "users_active";
}

impl ControllerUser {
    pub fn new(config: Arc<ConfigManager>, client: Arc<DaemonClient>) -> Self {
        Self { config, client }
    }

    /// Checks if user with username exists
    pub async fn exists(&self, username: String) -> Result<bool, error::ServiceError> {
        // Filter by username
        let filter = vec![("username", username)];
        let result = database::exists::<Self>(self.client.get_database().get_pool(), filter)
            .await
            .map_err(|e| error::ServiceError::Internal(e.to_string()))?;

        Ok(result)
    }

    /// Authenticate a given user.
    pub async fn auth(
        &self,
        username: String,
        password: String,
    ) -> Result<(String, String), error::ServiceError> {
        // Make sure that user with given username exists.
        if !self.exists(username.clone()).await? {
            return Err(error::ServiceError::NotFound(
                "could not find user, make sure username and password are correct".to_string(),
            ));
        }

        // Fetch user but only to get argon 2 parameters.
        let filter = vec![("username", username.to_string())];
        let user =
            database::read::<Self, schema::User>(self.client.get_database().get_pool(), filter)
                .await
                .map_err(|e| error::ServiceError::Internal(e.to_string()))?;

        // Generate password hash based on current password.
        let salt = user.salt;
        let password_hash = crypt::Argon2Factory::new(
            user.argon2_iters,
            user.argon2_memory_mb,
            user.argon2_parallelism,
        )
        .map_err(|e| error::ServiceError::Internal(e.to_string()))?
        .encode(password.as_bytes(), salt.as_bytes())
        .await
        .map_err(|e| error::ServiceError::Internal(e.to_string()))?;

        // Check if passwords match.
        if password_hash != user.password_hash {
            return Err(error::ServiceError::NotFound(
                "could not find user, make sure username and password are correct".to_string(),
            ));
        }

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

    /// Generate new auth token from refresh token.
    pub async fn refresh(&self, token_refresh: String) -> Result<String, error::ServiceError> {
        let jwt_factory = self.client.get_jwt_factory();

        // Make sure that current token is valid
        let claim_refresh = jwt_factory
            .decode::<crypt::JwtClaimRefresh>(&token_refresh)
            .map_err(|_| error::ServiceError::PermissionDenied("invalid refresh token".into()))?;

        // Fetch user based on uuid
        let uuid = uuid::Uuid::from_str(&claim_refresh.sub)
            .map_err(|e| error::ServiceError::Internal(e.to_string()))?;

        let filter = vec![("uuid", uuid.hyphenated().to_string())];
        let user =
            database::read::<Self, schema::User>(self.client.get_database().get_pool(), filter)
                .await
                .map_err(|e| error::ServiceError::Internal(e.to_string()))?;

        // Generate new auth token
        let token_auth = jwt_factory.encode(crypt::JwtClaimAccess::new(
            DaemonClient::ISSUER,
            user.uuid.into_uuid(),
            user.username,
        ));

        Ok(token_auth)
    }

    /// Add a new user.
    pub async fn add(
        &self,
        username: String,
        password: String,
    ) -> Result<(String, String), error::ServiceError> {
        // Make sure that user doesn't exist.
        // If there is a user with the same identifier error as each username is assumed to be unique.
        if self.exists(username.clone()).await? {
            return Err(error::ServiceError::AlreadyExists(format!(
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
        .map_err(|e| error::ServiceError::Internal(e.to_string()))?
        .encode(password.as_bytes(), salt.as_bytes())
        .await
        .map_err(|e| error::ServiceError::Internal(e.to_string()))?;

        let data = schema::User::new(
            username,
            password_hash,
            salt,
            config.argon2_iters,
            config.argon2_memory_mb,
            config.argon2_parallelism,
        )
        .map_err(|e| error::ServiceError::Internal(e.to_string()))?;

        // Try insert user into database and return auth tokens if successful.
        let user =
            database::create::<Self, schema::User>(self.client.get_database().get_pool(), data)
                .await
                .map_err(|e| error::ServiceError::Internal(e.to_string()))?;

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

#[cfg(test)]
mod tests {
    use crate::controller::ControllerUser;
    use crate::{client, config};

    use sqlx::sqlite;
    use std::sync::Arc;

    #[sqlx::test]
    async fn exists(pool: sqlite::SqlitePool) -> sqlx::Result<()> {
        let config = config::ConfigManager::mocked();
        let client = client::DaemonClient::mocked(pool)
            .await
            .expect("could not create mocked client");

        let controller = ControllerUser::new(Arc::new(config), Arc::new(client));

        // Should not exist before adding
        let result_not_exist = controller.exists("carl".to_string()).await;

        assert!(result_not_exist.is_ok());
        assert_eq!(result_not_exist.unwrap(), false);

        // Add something to the database
        let result_add = controller
            .add("carl".to_string(), "carl-loves-cars1234".to_string())
            .await;

        assert!(result_add.is_ok());

        // Should not exist since we added
        let result_exists = controller.exists("carl".to_string()).await;

        assert!(result_exists.is_ok());
        assert_eq!(result_exists.unwrap(), true);

        Ok(())
    }

    #[sqlx::test]
    async fn auth(pool: sqlite::SqlitePool) -> sqlx::Result<()> {
        let config = config::ConfigManager::mocked();
        let client = client::DaemonClient::mocked(pool)
            .await
            .expect("could not create mocked client");

        let controller = ControllerUser::new(Arc::new(config), Arc::new(client));

        // Add a new user to get tokens
        let result_add = controller
            .add("carl".to_string(), "carl-loves-cars1234".to_string())
            .await;

        assert!(result_add.is_ok());

        // Try auth with incorrect user
        let result_1 = controller
            .auth("bob".into(), "carl-loves-cars1234".to_string())
            .await;

        // Try auth with incorrect password
        let result_2 = controller
            .auth("carl".into(), "carl-loves-boats".to_string())
            .await;

        // Correct auth
        let result_ok = controller
            .auth("carl".into(), "carl-loves-cars1234".to_string())
            .await;

        assert!(result_1.is_err());
        assert!(result_2.is_err());
        assert!(result_ok.is_ok());

        Ok(())
    }

    #[sqlx::test]
    async fn add(pool: sqlite::SqlitePool) -> sqlx::Result<()> {
        let config = config::ConfigManager::mocked();
        let client = client::DaemonClient::mocked(pool)
            .await
            .expect("could not create mocked client");

        let controller = ControllerUser::new(Arc::new(config), Arc::new(client));

        let result_ok = controller
            .add("carl".to_string(), "carl-loves-cars1234".to_string())
            .await;

        // Duplicate entry should error
        let result_err_1 = controller
            .add("carl".into(), "carl-loves-boats".into())
            .await;

        // Name too short
        let result_err_2 = controller.add("c".into(), "hello".into()).await;

        // Name too long
        let result_err_3 = controller
            .add("a".repeat(256), "this-won't-work".into())
            .await;

        assert!(result_ok.is_ok());
        assert!(result_err_1.is_err());
        assert!(result_err_2.is_err());
        assert!(result_err_3.is_err());

        Ok(())
    }

    #[sqlx::test]
    async fn refresh(pool: sqlite::SqlitePool) -> sqlx::Result<()> {
        let config = config::ConfigManager::mocked();
        let client = client::DaemonClient::mocked(pool)
            .await
            .expect("could not create mocked client");

        let controller = ControllerUser::new(Arc::new(config), Arc::new(client));

        // Add a new user to get tokens
        let result_add = controller
            .add("carl".to_string(), "carl-loves-cars1234".to_string())
            .await;

        assert!(result_add.is_ok());

        let (token_auth, token_refresh) = result_add.unwrap();

        // Only refresh token can be used
        let result_1 = controller.refresh(token_auth).await;
        let result_2 = controller.refresh(token_refresh).await;

        assert!(result_1.is_err());
        assert!(result_2.is_ok());

        Ok(())
    }
}
