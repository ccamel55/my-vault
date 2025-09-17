use crate::error;

/// Custom message encode and decode trait.
pub trait MessageCode: Sized {
    /// Buffer size required to serialize data into.
    fn serialized_size(&self) -> Result<usize, error::Error>;

    /// Serialize message to binary.
    fn encode_to_slice(&self, buffer: &mut [u8]) -> Result<usize, error::Error>;

    /// Deserialize message from binary.
    fn decode_from_slice(buffer: &[u8]) -> Result<(Self, usize), error::Error>;
}
