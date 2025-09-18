use bincode::{Decode, Encode};

mod error;
mod traits;

pub mod message;
pub mod util;

pub use error::Error;
pub use traits::MessageCode;

/// Message type
#[derive(Clone, Debug, Decode, Encode)]
pub enum Message {
    /// Empty message
    NOP,

    /// Initiate connection shutdown
    Shutdown,

    /// Example Message
    Example(message::Example),

    /// Example Message 2
    Example2(message::Example2),
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl MessageCode for Message {
    fn serialized_size(&self) -> Result<usize, Error> {
        Ok(message::HEADER_EXPECTED_SIZE as usize + util::serialized_size(self)?)
    }

    fn encode_to_slice(&self, buffer: &mut [u8]) -> Result<usize, Error> {
        // Make sure we can store header data
        if buffer.len() < message::HEADER_EXPECTED_SIZE as usize {
            return Err(Error::BufferTooSmall);
        }

        let message_data = &mut buffer[message::HEADER_EXPECTED_SIZE as usize..];
        let message_size = util::encode_to_buffer(self, message_data)?;

        // Generate checksum
        let checksum = if message_size > 0 {
            util::MESSAGE_CRC_HASH.checksum(&message_data[0..message_size])
        } else {
            0
        };

        let header_data = &mut buffer[0..message::HEADER_EXPECTED_SIZE as usize];

        // Create correct message header
        let header = message::MessageHeader {
            message_size: message_size as u32,
            checksum,
            ..message::MessageHeader::default()
        };

        // Encode our header data
        util::encode_to_buffer(&header, header_data)?;

        Ok(message::HEADER_EXPECTED_SIZE as usize + message_size)
    }

    fn decode_from_slice(buffer: &[u8]) -> Result<(Self, usize), Error> {
        // Make sure we can store header data
        if buffer.len() < message::HEADER_EXPECTED_SIZE as usize {
            return Err(Error::BufferTooSmall);
        }

        let (header, header_size) = util::decode_from_buffer::<message::MessageHeader>(buffer)?;

        // Check that the message magic bytes exist.
        // Magic bytes indicate that the current buffered data is actually a message.
        if header.magic_bytes != message::HEADER_MAGIC_BYTE {
            return Err(Error::InvalidBuffer);
        }

        // Verify that the size of our header is the size that we read, otherwise
        // it suggests our data is corrupt or outdated.
        if header.header_size != message::HEADER_EXPECTED_SIZE
            || header.header_size as usize != header_size
        {
            return Err(Error::BufferCorrupt);
        }

        let message_data = &buffer[header.header_size as usize..];

        // If our message has a body check that out buffer is large enough to contain its data
        // and that the data has not been corrupted.
        if header.message_size > 0 {
            if message_data.len() < header.message_size as usize {
                return Err(Error::BufferTooSmall);
            }

            if util::MESSAGE_CRC_HASH.checksum(&message_data[0..header.message_size as usize])
                != header.checksum
            {
                return Err(Error::BufferCorrupt);
            }
        }

        let (message, size) = util::decode_from_buffer::<Self>(message_data)?;

        if size != header.message_size as usize {
            return Err(Error::BufferCorrupt);
        }

        Ok((message, size))
    }
}
