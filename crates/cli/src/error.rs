use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("argument error - {0}")]
    Argument(String),

    #[error("connection error - {0}")]
    Connection(String),

    #[error("console io error - {0}")]
    IO(String),

    #[error("rpc status {0}: {1}")]
    Tonic(i32, String),
}

/// Convert io error to error
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value.to_string())
    }
}

/// Convert tonic status to error
impl From<tonic::Status> for Error {
    fn from(value: tonic::Status) -> Self {
        Self::Tonic(value.code() as i32, value.message().into())
    }
}
