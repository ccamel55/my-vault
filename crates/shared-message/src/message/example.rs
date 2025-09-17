use bincode::{Decode, Encode};

/// Example message data
#[derive(Clone, Debug, Decode, Encode)]
pub struct Example {
    pub value_1: i32,
    pub value_2: f32,
    pub value_3: i64,
    pub value_4: f64,
}

/// Example message data
#[derive(Clone, Debug, Decode, Encode)]
pub struct Example2 {
    pub value_1: String,
    pub value_2: [i32; 4],
}
