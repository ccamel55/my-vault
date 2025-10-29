mod client;
mod collection;
mod secret;
mod source;
mod user;

pub use client::*;
pub use collection::*;
pub use secret::*;
pub use user::*;

/// Error which represents status code.
/// Add more as needed.
#[derive(thiserror::Error, Debug, Clone)]
pub enum ControllerError {
    /// `tonic::status::Code::Cancelled`
    #[error("{0}")]
    Cancelled(String),

    /// `tonic::status::Code::Unknown`
    #[error("{0}")]
    Unknown(String),

    /// `tonic::status::Code::AlreadyExists`
    #[error("{0}")]
    AlreadyExists(String),

    /// `tonic::status::Code::PermissionDenied`
    #[error("{0}")]
    PermissionDenied(String),

    /// `tonic::status::Code::Internal`
    #[error("{0}")]
    Internal(String),

    /// `tonic::status::Code::NotFound`
    #[error("{0}")]
    NotFound(String),
}
