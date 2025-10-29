use validator::Validate;

/// User row entry
#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow, validator::Validate)]
pub struct User {
    #[serde(deserialize_with = "shared_core::serde::uuid::Hyphenated::deserialize")]
    pub uuid: uuid::fmt::Hyphenated,
    #[validate(length(min = 3, max = 255))]
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub argon2_iters: u32,
    pub argon2_memory_mb: u32,
    pub argon2_parallelism: u32,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted: Option<bool>,
}

impl User {
    pub fn new<A, B, C>(
        username: A,
        password_hash: B,
        salt: C,
        argon2_iters: u32,
        argon2_memory_mb: u32,
        argon2_parallelism: u32,
    ) -> Result<Self, validator::ValidationErrors>
    where
        A: ToString,
        B: ToString,
        C: ToString,
    {
        let res = User {
            uuid: uuid::Uuid::new_v4().into(),
            username: username.to_string(),
            password_hash: password_hash.to_string(),
            salt: salt.to_string(),
            argon2_iters,
            argon2_memory_mb,
            argon2_parallelism,
            created_at: None,
            updated_at: None,
            deleted: None,
        };

        res.validate()?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::User;
    use shared_core::rng;

    #[tokio::test]
    async fn new_user() {
        let res_ok = User::new(
            "hello@mail.com",
            "123456",
            rng::random_bytes_str(16),
            2,
            32,
            2,
        );

        assert!(res_ok.is_ok());

        let res_username_to_short = User::new("h", "123456", rng::random_bytes_str(16), 2, 32, 2);

        assert!(res_username_to_short.is_err());

        let res_username_to_short = res_username_to_short.unwrap_err();
        let res_username_to_short = res_username_to_short.field_errors();

        assert!(res_username_to_short.contains_key("username"));

        let res_username_to_long = User::new(
            "a".repeat(256),
            "123456",
            rng::random_bytes_str(16),
            2,
            32,
            2,
        );

        assert!(res_username_to_long.is_err());

        let res_username_to_long = res_username_to_long.unwrap_err();
        let res_username_to_long = res_username_to_long.field_errors();

        assert!(res_username_to_long.contains_key("username"));
    }
}
