use crate::controller;
use crate::error;

use poem::EndpointExt;
use std::sync::Arc;

mod client;
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

    let service_client = client::ClientService::new(controller_client)?;
    let service_user = user::UserService::new(controller_user)?;

    // Create API endpoints
    const SERVICE_PATH_PREFIX: &str = "/api/v1";

    let services = (service_client, service_user);
    let services = poem_openapi::OpenApiService::new(services, "My Vault", "0.1.0")
        .url_prefix(SERVICE_PATH_PREFIX);

    // Create a router which will handle the correct services
    let route = if enable_ui {
        // Create route with ui endpoint
        poem::Route::new().nest("/", services.scalar())
    } else {
        poem::Route::new()
    }
    .nest(SERVICE_PATH_PREFIX, services)
    .data(client.clone());

    Ok(route)
}
