mod auth;
mod client;

use shared_service::{auth_server, client_server};
use std::sync::Arc;

/// Create gRPC service router with all our services.
pub async fn create_services(
    client: Arc<crate::DaemonClient>,
) -> anyhow::Result<tonic::service::Routes> {
    let mw_auth = crate::middleware::Authentication::new(client.clone())?;

    let service_auth = auth::AuthService::new(client.clone())?;
    let service_client = client::ClientService::new(client.clone())?;

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(shared_service::FILE_DESCRIPTOR_SET)
        .build_v1alpha()?;

    let (health_reporter, health_service) = tonic_health::server::health_reporter();

    health_reporter
        .set_serving::<client_server::ClientServer<client::ClientService>>()
        .await;

    health_reporter
        .set_serving::<auth_server::AuthServer<auth::AuthService>>()
        .await;

    let routes = tonic::service::RoutesBuilder::default()
        .add_service(reflection_service)
        .add_service(health_service)
        .add_service(auth_server::AuthServer::new(service_auth))
        .add_service(tonic_middleware::InterceptorFor::new(
            client_server::ClientServer::new(service_client),
            mw_auth.clone(),
        ))
        .to_owned()
        .routes();

    Ok(routes)
}
