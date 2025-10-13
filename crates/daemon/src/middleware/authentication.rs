use tonic::Status;
use tonic::body::Body;
use tonic::codegen::http::Request;

#[derive(Debug, Clone)]
pub struct Authentication;

impl Authentication {
    /// Create new authentication middleware instance.
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {})
    }
}

#[tonic::async_trait]
impl tonic_middleware::RequestInterceptor for Authentication {
    async fn intercept(&self, mut req: Request<Body>) -> Result<Request<Body>, Status> {
        // TODO: verify JWT lol
        req.extensions_mut()
            .insert(crate::middleware::RequestExtension {});

        Ok(req)
    }
}
