mod client;
mod collection;
mod secrets;
mod user;

pub use client::*;
pub use collection::*;
pub use secrets::*;
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
}
