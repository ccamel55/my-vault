use shared_message::{Message, message, util};

#[test]
fn message_compare() {
    let message_1 = Message::NOP;
    let message_2 = Message::Shutdown;
    let message_3 = Message::Example(message::Example::default());
    let message_4 = Message::Example(message::Example {
        value_1: 100,
        ..message::Example::default()
    });

    assert_eq!(message_1, Message::NOP);
    assert_eq!(message_2, Message::Shutdown);

    assert_ne!(message_2, message_3);
    assert_eq!(message_3, message_4);
}

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
    let mut buffer = vec![0; message::HEADER_EXPECTED_SIZE as usize + 4];

    let data = Message::NOP;
    let size = data.encode_to_slice(&mut buffer).unwrap();

    let buffer_1 = buffer[0..message::HEADER_EXPECTED_SIZE as usize - 1].to_owned();
    let buffer_2 = buffer[0..size - 1].to_owned();

    let data_decode_1 = Message::decode_from_slice(&buffer_1);
    let data_decode_2 = Message::decode_from_slice(&buffer_2);

    assert!(data_decode_1.is_err());
    assert!(data_decode_2.is_err());
}

#[test]
fn message_decode_invalid_data() {
    let mut buffer = vec![0; message::HEADER_EXPECTED_SIZE as usize + 4];

    let data = Message::NOP;
    let _size = data.encode_to_slice(&mut buffer).unwrap();

    let (header, _size) = util::decode_from_buffer::<message::MessageHeader>(&buffer).unwrap();

    // Invalid magic byte
    {
        let mut header = header.clone();
        header.magic_bytes = [0, 0, 0];

        util::encode_to_buffer(&header, &mut buffer).unwrap();

        let message = Message::decode_from_slice(&buffer);

        assert!(message.is_err());
    }

    // Invalid header size
    {
        let mut header = header.clone();
        header.message_size = 69;

        util::encode_to_buffer(&header, &mut buffer).unwrap();

        let message = Message::decode_from_slice(&buffer);

        assert!(message.is_err());
    }

    // Invalid checksum
    {
        let mut header = header.clone();
        header.checksum = 0;

        util::encode_to_buffer(&header, &mut buffer).unwrap();

        let message = Message::decode_from_slice(&buffer);

        assert!(message.is_err());
    }
}

#[test]
fn message_encode_too_small() {
    let mut buffer_1 = vec![0; message::HEADER_EXPECTED_SIZE as usize - 1];
    let mut buffer_2 = vec![0; message::HEADER_EXPECTED_SIZE as usize + 4 - 1];

    let data = Message::NOP;

    let data_1 = data.encode_to_slice(&mut buffer_1);
    let data_2 = data.encode_to_slice(&mut buffer_2);

    assert!(data_1.is_err());
    assert!(data_2.is_err());
}
