use shared_message;
use shared_message::{Error, message, util};

#[test]
fn error_compare() {
    let error_1 = Error::BufferTooSmall;
    let error_2 = Error::BufferCorrupt;
    let error_3 = Error::DecodeError(bincode::error::DecodeError::LimitExceeded);
    let error_4 = Error::DecodeError(bincode::error::DecodeError::UnexpectedEnd { additional: 0 });
    let error_5 = Error::DecodeError(bincode::error::DecodeError::UnexpectedEnd { additional: 1 });

    assert_eq!(error_1, Error::BufferTooSmall);
    assert_eq!(error_2, Error::BufferCorrupt);

    assert_ne!(error_2, error_3);

    assert_eq!(error_3, error_4);
    assert_eq!(error_3, error_5);
}

#[test]
fn crc32_checksum() {
    const EXPECTED_CHECKSUM: u32 = 2683815467;

    let example_data: Vec<u8> = vec![1, 2, 3, 4, 5, 99, 98, 97];

    let checksum_1 = util::MESSAGE_CRC_HASH.checksum(&example_data);
    let checksum_2 = util::MESSAGE_CRC_HASH.checksum(&example_data);

    assert_eq!(checksum_1, EXPECTED_CHECKSUM);
    assert_eq!(checksum_2, EXPECTED_CHECKSUM);
}

#[test]
fn message_header_encode() {
    let header = message::MessageHeader::default();

    let mut buffer = vec![0; 1024];

    let size = util::encode_to_buffer(&header, &mut buffer);
    let size_expected = util::serialized_size(&header);

    assert!(size.is_ok());
    assert!(size_expected.is_ok());

    let size = size.unwrap();
    let size_expected = size_expected.unwrap();

    assert_eq!(header.header_size, message::HEADER_EXPECTED_SIZE);

    assert_eq!(size, header.header_size as usize);
    assert_eq!(size_expected, header.header_size as usize);
}

#[test]
fn message_header_encode_too_small() {
    let header = message::MessageHeader::default();

    let mut buffer = vec![0; (message::HEADER_EXPECTED_SIZE - 1) as usize];
    let size = util::encode_to_buffer(&header, &mut buffer);

    assert_eq!(size, Err(Error::BufferTooSmall));
}

#[test]
fn message_header_decode() {
    let header_bytes: Vec<u8> = vec![42, 57, 52, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let header_bytes_random: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    let header = util::decode_from_buffer::<message::MessageHeader>(&header_bytes);
    let header_random = util::decode_from_buffer::<message::MessageHeader>(&header_bytes_random);

    assert!(header.is_ok());
    assert!(header_random.is_ok());

    let (header, size) = header.unwrap();

    assert_eq!(size, message::HEADER_EXPECTED_SIZE as usize);

    assert_eq!(header.magic_bytes, message::HEADER_MAGIC_BYTE);
    assert_eq!(header.header_size, message::HEADER_EXPECTED_SIZE);

    let (header_random, size_random) = header_random.unwrap();

    assert_eq!(size_random, message::HEADER_EXPECTED_SIZE as usize);

    assert_ne!(header_random.magic_bytes, message::HEADER_MAGIC_BYTE);
    assert_ne!(header_random.header_size, message::HEADER_EXPECTED_SIZE);
}

#[test]
fn message_header_decode_large_buffer() {
    let header_bytes: Vec<u8> = vec![
        42, 57, 52, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6,
    ];

    let header = util::decode_from_buffer::<message::MessageHeader>(&header_bytes);

    assert!(header.is_ok());

    let (header, size) = header.unwrap();

    assert_eq!(size, message::HEADER_EXPECTED_SIZE as usize);

    assert_eq!(header.magic_bytes, message::HEADER_MAGIC_BYTE);
    assert_eq!(header.header_size, message::HEADER_EXPECTED_SIZE);
}

#[test]
fn message_header_decode_to_small() {
    let header_bytes: Vec<u8> = vec![];
    let header = util::decode_from_buffer::<message::MessageHeader>(&header_bytes);

    assert!(header.is_err());

    if let Err(Error::DecodeError(_)) = header {
        assert!(true)
    } else {
        assert!(false)
    }
}
