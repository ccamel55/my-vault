use crate::{controller, middleware};

use poem_openapi::payload::Json;
use poem_openapi::{Object, OpenApi};

#[derive(Debug, Clone)]
pub struct ClientService {
    controller_client: controller::ControllerClient,
}

impl ClientService {
    pub fn new(controller_client: controller::ControllerClient) -> anyhow::Result<Self> {
        Ok(Self {
            controller_client: controller_client.clone(),
        })
    }
}

/// Info response - GET
#[derive(Debug, Clone, Object)]
struct InfoResponseGet {
    uptime_seconds: u64,
}

#[OpenApi(prefix_path = "/client")]
impl ClientService {
    /// Get info about client.
    #[oai(path = "/info", method = "get")]
    async fn info_get(
        &self,
        _user: middleware::JwtAuthorization,
    ) -> Result<Json<InfoResponseGet>, super::Error> {
        let time_start = *self.controller_client.client.get_time_started();
        let time_elapsed = chrono::Utc::now() - time_start;

        let res = InfoResponseGet {
            uptime_seconds: time_elapsed.num_seconds() as u64,
        };

        Ok(Json(res))
    }
}
