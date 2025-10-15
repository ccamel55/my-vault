#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub uuid: uuid::Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
}
