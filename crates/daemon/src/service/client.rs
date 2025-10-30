use crate::{controller, middleware};

use poem_openapi::payload::Json;
use poem_openapi::{Object, OpenApi};

#[derive(Debug, Clone)]
pub struct ClientService {
    controller: controller::ControllerClient,
}

impl ClientService {
    pub fn new(controller: controller::ControllerClient) -> Self {
        Self { controller }
    }
}

/// Info response - GET
#[derive(Debug, Clone, Object)]
struct InfoResponseGet {
    uptime_seconds: u64,
}

#[OpenApi(prefix_path = "/client")]
impl ClientService {
    /// Client Info
    #[oai(path = "/info", method = "get")]
    async fn client_info(
        &self,
        _user: middleware::JwtAuthorization,
    ) -> poem::Result<Json<InfoResponseGet>> {
        let uptime_seconds = self.controller.uptime_seconds()?;
        let res = InfoResponseGet { uptime_seconds };

        Ok(Json(res))
    }
}
