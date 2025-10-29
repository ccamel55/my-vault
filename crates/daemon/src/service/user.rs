use crate::{controller, middleware};

use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use poem_openapi::{Object, OpenApi};

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

/// Login request - POST
#[derive(Debug, Clone, Object)]
struct LoginRequestPost {
    username: String,
    password: String,
}

/// Login response - POST
#[derive(Debug, Clone, Object)]
struct LoginResponsePost {
    token_auth: String,
    token_refresh: String,
}

/// Refresh request - POST
#[derive(Debug, Clone, Object)]
struct RefreshRequestPost {
    token_refresh: String,
}

/// Refresh response - POST
#[derive(Debug, Clone, Object)]
struct RefreshResponsePost {
    token_auth: String,
}

/// User request - POST
#[derive(Debug, Clone, Object)]
struct UserRequestPost {
    username: String,
    password: String,
}

/// User response - POST
#[derive(Debug, Clone, Object)]
struct UserResponsePost {
    token_auth: String,
    token_refresh: String,
}

/// User response - GET
#[derive(Debug, Clone, Object)]
struct UserResponseGet {
    uuid: uuid::Uuid,
    name: String,
}

#[OpenApi(prefix_path = "/user")]
impl UserService {
    /// Login with existing user credentials
    #[oai(path = "/login", method = "post")]
    async fn login(
        &self,
        request: Json<LoginRequestPost>,
    ) -> poem::Result<Json<LoginResponsePost>> {
        let request = request.0;

        let (token_auth, token_refresh) = self
            .controller_user
            .auth(request.username, request.password)
            .await?;

        let res = LoginResponsePost {
            token_auth,
            token_refresh,
        };

        Ok(Json(res))
    }

    /// Generate a new auth token from an existing refresh token
    #[oai(path = "/refresh", method = "post")]
    async fn refresh(
        &self,
        request: Json<RefreshRequestPost>,
    ) -> poem::Result<Json<RefreshResponsePost>> {
        let request = request.0;

        let token_auth = self.controller_user.refresh(request.token_refresh).await?;
        let res = RefreshResponsePost { token_auth };

        Ok(Json(res))
    }

    /// Register a new user.
    #[oai(path = "/", method = "post")]
    async fn user_create(
        &self,
        request: Json<UserRequestPost>,
    ) -> poem::Result<Json<UserResponsePost>> {
        let request = request.0;

        let (token_auth, token_refresh) = self
            .controller_user
            .add(request.username, request.password)
            .await?;

        let res = UserResponsePost {
            token_auth,
            token_refresh,
        };

        Ok(Json(res))
    }

    /// Get information about a given user.
    /// note: This endpoint requires a valid auth token.
    #[oai(path = "/:username", method = "get")]
    async fn user_info(
        &self,
        _user: middleware::JwtAuthorization,
        _username: Path<String>,
    ) -> poem::Result<Json<UserResponseGet>> {
        Err(poem::Error::from_status(
            poem::http::StatusCode::NOT_IMPLEMENTED,
        ))
    }

    /// Delete a given user.
    /// note: This endpoint requires a valid auth token.
    #[oai(path = "/:username", method = "delete")]
    async fn user_delete(
        &self,
        _user: middleware::JwtAuthorization,
        _username: Path<String>,
    ) -> poem::Result<()> {
        Err(poem::Error::from_status(
            poem::http::StatusCode::NOT_IMPLEMENTED,
        ))
    }
}
