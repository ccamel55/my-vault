use crate::controller;

use poem::EndpointExt;
use std::sync::Arc;

mod client;
mod user;

impl From<controller::ControllerError> for poem::Error {
    fn from(value: controller::ControllerError) -> Self {
        match value {
            controller::ControllerError::AlreadyExists(x) => {
                Self::from_string(x, poem::http::StatusCode::CONFLICT)
            }
            controller::ControllerError::PermissionDenied(x) => {
                Self::from_string(x, poem::http::StatusCode::FORBIDDEN)
            }
            controller::ControllerError::Internal(x) => {
                Self::from_string(x, poem::http::StatusCode::INTERNAL_SERVER_ERROR)
            }
            controller::ControllerError::NotFound(x) => {
                Self::from_string(x, poem::http::StatusCode::NOT_FOUND)
            }
        }
    }
}

/// Create services
pub async fn create_services(
    config: Arc<crate::ConfigManager>,
    client: Arc<crate::DaemonClient>,
) -> anyhow::Result<impl poem::Endpoint> {
    let controller_client = controller::ControllerClient::new(client.clone());
    let controller_user = controller::ControllerUser::new(config.clone(), client.clone());

    let service_client = client::ClientService::new(controller_client)?;
    let service_user = user::UserService::new(controller_user)?;

    // Create API endpoints
    let services = (service_client, service_user);
    let services =
        poem_openapi::OpenApiService::new(services, "My Vault", "0.1.0").url_prefix("/api/v1");

    // Create UI endpoint
    let docs_ui = services.scalar();

    // Create a router which will handle the correct services
    let route = poem::Route::new()
        .nest("/api/v1", services)
        .nest("/", docs_ui)
        .data(client.clone());

    Ok(route)
}
