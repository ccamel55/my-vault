use poem_openapi::payload::Json;
use poem_openapi::{Object, OpenApi};

#[derive(Debug, Clone)]
pub struct HealthService;

impl HealthService {
    pub fn new() -> Self {
        Self {}
    }
}

/// Health response - GET
#[derive(Debug, Clone, Object)]
struct HealthResponseGet {
    healthy: bool,
}

#[OpenApi]
impl HealthService {
    /// Health check
    #[oai(path = "/health", method = "get")]
    async fn health_check(&self) -> poem::Result<Json<HealthResponseGet>> {
        let res = HealthResponseGet { healthy: true };

        Ok(Json(res))
    }
}
