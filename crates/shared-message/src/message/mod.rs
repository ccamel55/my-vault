mod example;

use bincode::{Decode, Encode};

pub use example::{Example, Example2};

/// Magic bytes to indicate the start of a message.
pub const HEADER_MAGIC_BYTE: [u8; 3] = [42, 57, 52];

/// Expected size of message headers
pub const HEADER_EXPECTED_SIZE: u8 = 12;

/// Header for all messages
#[derive(Clone, Debug, Encode, Decode)]
pub struct MessageHeader {
    /// Magic bytes that indicate the start of a message.
    pub magic_bytes: [u8; 3],

    /// Length of message header.
    pub header_size: u8,

    /// Length of message excluding header.
    pub message_size: u32,

    /// CRC32 checksum of message excluding header.
    /// This is used to verify the body of our message is not corrupt.
    pub checksum: u32,
}

impl Default for MessageHeader {
    fn default() -> Self {
        Self {
            magic_bytes: HEADER_MAGIC_BYTE,
            header_size: HEADER_EXPECTED_SIZE,
            message_size: 0,
            checksum: 0,
        }
    }
}
