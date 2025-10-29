use crate::controller;

use poem::EndpointExt;
use poem_openapi::ApiResponse;
use poem_openapi::payload::PlainText;
use std::sync::Arc;

mod client;
mod user;

/// Service error codes.
/// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status.
#[derive(ApiResponse, Debug, Clone)]
pub enum Error {
    /// The server cannot or will not process the request due to something that is perceived to be a client error
    /// (e.g., malformed request syntax, invalid request message framing, or deceptive request routing).
    #[oai(status = 400)]
    BadRequest(PlainText<String>),

    /// Although the HTTP standard specifies "unauthorized", semantically this response means "unauthenticated".
    /// That is, the client must authenticate itself to get the requested response.
    #[oai(status = 401)]
    Unauthorized(PlainText<String>),

    /// The client does not have access rights to the content; that is, it is unauthorized, so the server is
    /// refusing to give the requested resource. Unlike 401 Unauthorized, the client's identity is known
    /// to the server.
    #[oai(status = 403)]
    Forbidden(PlainText<String>),

    /// The server cannot find the requested resource. In an API, this can mean that the endpoint is valid
    /// but the resource itself does not exist. Servers may also send this response instead of 403 Forbidden
    /// to hide the existence of a resource from an unauthorized client.
    #[oai(status = 404)]
    NotFound(PlainText<String>),

    /// This response is sent when a request which conflicts with the current state of the server.
    #[oai(status = 409)]
    Conflict(PlainText<String>),

    /// The user has sent too many requests in a given amount of time (rate limiting).
    #[oai(status = 429)]
    TooManyRequests,

    /// The server has encountered a situation it does not know how to handle. This error is generic,
    /// indicating that the server cannot find a more appropriate 5XX status code to respond with.
    #[oai(status = 500)]
    InternalServerError(PlainText<String>),

    /// The request method is not supported by the server and cannot be handled. The only methods that
    /// servers are required to support (and therefore that must not return this code) are GET and HEAD.
    #[oai(status = 501)]
    NotImplemented,
}

impl From<controller::ControllerError> for Error {
    fn from(value: controller::ControllerError) -> Self {
        match value {
            controller::ControllerError::AlreadyExists(x) => Self::Conflict(PlainText(x)),
            controller::ControllerError::PermissionDenied(x) => Self::Forbidden(PlainText(x)),
            controller::ControllerError::Internal(x) => Self::InternalServerError(PlainText(x)),
            controller::ControllerError::NotFound(x) => Self::NotFound(PlainText(x)),
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
