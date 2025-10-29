mod client;
mod collection;
mod secret;
mod source;
mod user;

pub use client::*;
pub use collection::*;
pub use secret::*;
pub use user::*;

/// Controller Errors  
#[derive(thiserror::Error, Debug, Clone)]
pub enum ControllerError {
    #[error("{0}")]
    AlreadyExists(String),

    #[error("{0}")]
    PermissionDenied(String),

    #[error("{0}")]
    Internal(String),

    #[error("{0}")]
    NotFound(String),
}
