use crate::error;
use crate::{controller, middleware};

use poem::EndpointExt;
use std::sync::Arc;

mod client;
mod health;
mod user;

/// Convert from controller error to status.
impl From<error::ServiceError> for poem::Error {
    fn from(value: error::ServiceError) -> Self {
        match value {
            error::ServiceError::AlreadyExists(x) => {
                Self::from_string(x, poem::http::StatusCode::CONFLICT)
            }
            error::ServiceError::PermissionDenied(x) => {
                Self::from_string(x, poem::http::StatusCode::FORBIDDEN)
            }
            error::ServiceError::Internal(x) => {
                Self::from_string(x, poem::http::StatusCode::INTERNAL_SERVER_ERROR)
            }
            error::ServiceError::NotFound(x) => {
                Self::from_string(x, poem::http::StatusCode::NOT_FOUND)
            }
        }
    }
}

/// Create services
pub async fn create_services(
    enable_ui: bool,
    config: Arc<crate::ConfigManager>,
    client: Arc<crate::DaemonClient>,
) -> anyhow::Result<impl poem::Endpoint> {
    let controller_client = controller::ControllerClient::new(client.clone());
    let controller_user = controller::ControllerUser::new(config.clone(), client.clone());

    // Create data to be injected
    let middleware_data = middleware::MiddlewareData::new(config, client);

    // Create API endpoints
    const SERVICE_PATH_PREFIX: &str = "/api/v1";

    let services = (
        health::HealthService::new(),
        client::ClientService::new(controller_client),
        user::UserService::new(controller_user),
    );

    let api = poem_openapi::OpenApiService::new(services, "My Vault", "0.1.0")
        .url_prefix(SERVICE_PATH_PREFIX);

    // Create a router which will handle the correct services
    let route = if enable_ui {
        // Create route with ui endpoint
        poem::Route::new().nest("/", api.scalar())
    } else {
        poem::Route::new()
    }
    .nest(
        SERVICE_PATH_PREFIX,
        api.with(middleware::SetDefaultHeader::new())
            .with(poem::middleware::AddData::new(middleware_data)),
    );

    Ok(route)
}
