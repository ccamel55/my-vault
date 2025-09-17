use shared_message::{Message, MessageCode, message};

#[test]
fn message_encode_decode() {
    let mut buffer_1 = vec![0; message::HEADER_EXPECTED_SIZE as usize + 4];
    let mut buffer_2 = vec![0; message::HEADER_EXPECTED_SIZE as usize + 28];

    let example_data = message::Example {
        value_1: 69420,
        value_2: 1.23,
        value_3: -121243,
        value_4: -12.340,
    };

    let data_1 = Message::NOP;
    let data_2 = Message::Example(example_data.clone());

    let size_1 = data_1.encode_to_slice(&mut buffer_1);
    let size_2 = data_2.encode_to_slice(&mut buffer_2);

    assert!(size_1.is_ok());
    assert!(size_2.is_ok());

    assert_eq!(size_1.unwrap(), message::HEADER_EXPECTED_SIZE as usize + 4);
    assert_eq!(size_2.unwrap(), message::HEADER_EXPECTED_SIZE as usize + 28);

    let data_1_decode = Message::decode_from_slice(&buffer_1);
    let data_2_decode = Message::decode_from_slice(&buffer_2);

    assert!(data_1_decode.is_ok());
    assert!(data_2_decode.is_ok());

    let (data_1_decode, _size) = data_1_decode.unwrap();
    let (data_2_decode, _size) = data_2_decode.unwrap();

    if let Message::NOP = data_1_decode {
        assert!(true)
    } else {
        assert!(false)
    }

    if let Message::Example(data) = data_2_decode {
        assert_eq!(data.value_1, example_data.value_1);
        assert_eq!(data.value_2, example_data.value_2);
        assert_eq!(data.value_3, example_data.value_3);
        assert_eq!(data.value_4, example_data.value_4);
    } else {
        assert!(false)
    }

    // Make sure decode also works if we are reading from a buffer that is larger than the message size

    buffer_1.push(0);
    buffer_1.push(0);
    buffer_1.push(0);

    buffer_2.push(0);
    buffer_2.push(0);
    buffer_2.push(0);

    let data_1_decode = Message::decode_from_slice(&buffer_1);
    let data_2_decode = Message::decode_from_slice(&buffer_2);

    assert!(data_1_decode.is_ok());
    assert!(data_2_decode.is_ok());
}

#[test]
fn message_decode_too_small() {
    // Smaller than header
    // Smaller than data
}

#[test]
fn message_decode_invalid_data() {
    // Invalid magic byte
    // Invalid header size
}

#[test]
fn message_decode_corrupt() {
    // Checksum wrong
    // invalid message integer representation
}

#[test]
fn message_encode_too_small() {
    // Smaller than header
    // Smaller than data
}
