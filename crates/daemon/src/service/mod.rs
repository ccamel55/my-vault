mod client;
mod user;

use crate::controller;

use shared_service::{client_server, user_server};
use std::sync::Arc;

/// Create gRPC service router with all our services.
pub async fn create_services(
    config: Arc<crate::ConfigManager>,
    client: Arc<crate::DaemonClient>,
) -> anyhow::Result<tonic::service::Routes> {
    let mw_auth = crate::middleware::Authentication::new(client.clone())?;

    let controller_client = controller::ControllerClient::new(client.clone());
    let controller_user = controller::ControllerUser::new(config.clone(), client.clone());

    let service_client = client::ClientService::new(controller_client)?;
    let service_user = user::UserService::new(controller_user)?;

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(shared_service::FILE_DESCRIPTOR_SET)
        .build_v1alpha()?;

    let (health_reporter, health_service) = tonic_health::server::health_reporter();

    health_reporter
        .set_serving::<client_server::ClientServer<client::ClientService>>()
        .await;

    health_reporter
        .set_serving::<user_server::UserServer<user::UserService>>()
        .await;

    let routes = tonic::service::RoutesBuilder::default()
        .add_service(reflection_service)
        .add_service(health_service)
        .add_service(user_server::UserServer::new(service_user))
        .add_service(tonic_middleware::InterceptorFor::new(
            client_server::ClientServer::new(service_client),
            mw_auth.clone(),
        ))
        .to_owned()
        .routes();

    Ok(routes)
}

impl From<controller::ControllerError> for tonic::Status {
    fn from(value: controller::ControllerError) -> Self {
        match value {
            controller::ControllerError::Cancelled(x) => Self::cancelled(x),
            controller::ControllerError::Unknown(x) => Self::unknown(x),
            controller::ControllerError::AlreadyExists(x) => Self::already_exists(x),
            controller::ControllerError::PermissionDenied(x) => Self::permission_denied(x),
            controller::ControllerError::Internal(x) => Self::internal(x),
            controller::ControllerError::NotFound(x) => Self::not_found(x),
        }
    }
}
