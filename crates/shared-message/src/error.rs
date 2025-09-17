use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Buffer is to small to read/write data")]
    BufferTooSmall,

    #[error("Buffer data is corrupt")]
    BufferCorrupt,

    #[error("Decode error: {0}")]
    DecodeError(bincode::error::DecodeError),

    #[error("Encode error: {0}")]
    EncodeError(bincode::error::EncodeError),

    #[error("Buffer does not represent a valid message")]
    InvalidBuffer,
}
