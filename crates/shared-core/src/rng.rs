use rand::Rng;

/// Get random string sequence.
pub fn random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// Get random byte sequence
pub fn random_bytes(length: usize) -> Vec<u8> {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(length)
        .collect()
}
