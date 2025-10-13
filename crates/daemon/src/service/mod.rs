mod client;
mod user;

use shared_service::{client_server, user_server};
use std::sync::Arc;

/// Create gRPC service router with all our services.
pub async fn create_services(
    client: Arc<crate::DaemonClient>,
) -> anyhow::Result<tonic::service::Routes> {
    let mw_auth = crate::middleware::Authentication::new()?;

    let service_client = client::ClientService::new(client.clone())?;
    let service_user = user::UserService::new(client.clone())?;

    let (health_reporter, health_service) = tonic_health::server::health_reporter();

    health_reporter
        .set_serving::<client_server::ClientServer<client::ClientService>>()
        .await;

    health_reporter
        .set_serving::<user_server::UserServer<user::UserService>>()
        .await;

    let routes = tonic::service::RoutesBuilder::default()
        .add_service(health_service)
        .add_service(tonic_middleware::InterceptorFor::new(
            client_server::ClientServer::new(service_client),
            mw_auth.clone(),
        ))
        .add_service(tonic_middleware::InterceptorFor::new(
            user_server::UserServer::new(service_user),
            mw_auth.clone(),
        ))
        .to_owned()
        .routes();

    Ok(routes)
}
