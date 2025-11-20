/// Service Errors
#[derive(thiserror::Error, Debug, Clone)]
pub enum ServiceError {
    #[error("{0}")]
    AlreadyExists(String),

    #[error("{0}")]
    PermissionDenied(String),

    #[error("{0}")]
    Internal(String),

    #[error("{0}")]
    NotFound(String),
}
