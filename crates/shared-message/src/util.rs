use crate::error;

/// Bin code encoding/decoding config.
/// Changing this is considered a breaking change.
pub type BincodeConfig =
    bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint>;

/// CRC32 implementation for messages.
pub const MESSAGE_CRC_HASH: crc::Crc<u32, crc::Table<16>> =
    crc::Crc::<u32, crc::Table<16>>::new(&crc::CRC_32_ISCSI);

/// Get the size of a type when serialized.
pub fn serialized_size<E: bincode::enc::Encode>(value: E) -> Result<usize, error::Error> {
    let mut writer = bincode::enc::write::SizeWriter::default();

    bincode::encode_into_writer(&value, &mut writer, BincodeConfig::default())
        .map_err(error::Error::EncodeError)?;

    Ok(writer.bytes_written)
}

/// Encode a struct into an existing buffer.
pub fn encode_to_buffer<E: bincode::enc::Encode>(
    value: E,
    buffer: &mut [u8],
) -> Result<usize, error::Error> {
    let size = serialized_size(&value)?;

    if buffer.len() < size {
        return Err(error::Error::BufferTooSmall);
    }

    bincode::encode_into_slice(&value, buffer, BincodeConfig::default())
        .map_err(error::Error::EncodeError)
}

/// Decode data from a buffer into a struct.
pub fn decode_from_buffer<D: bincode::de::Decode<()>>(
    buffer: &[u8],
) -> Result<(D, usize), error::Error> {
    bincode::decode_from_slice(buffer, BincodeConfig::default()).map_err(error::Error::DecodeError)
}
