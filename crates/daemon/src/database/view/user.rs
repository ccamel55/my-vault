use validator::Validate;

/// User row entry
#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow, validator::Validate)]
pub struct User {
    #[serde(deserialize_with = "shared_core::serde::uuid::Hyphenated::deserialize")]
    pub uuid: uuid::fmt::Hyphenated,
    #[validate(email)]
    pub email: String,
    pub password_hash: String,
    #[validate(length(min = 2, max = 255))]
    pub first_name: String,
    #[validate(length(min = 2, max = 255))]
    pub last_name: String,
    pub salt: Vec<u8>,
    pub argon2_iters: u32,
    pub argon2_memory_mb: u32,
    pub argon2_parallelism: u32,
    pub last_updated: Option<chrono::NaiveDateTime>,
}

impl User {
    pub fn new<A, B, C, D>(
        email: A,
        password_hash: B,
        first_name: C,
        last_name: D,
        salt: Vec<u8>,
        argon2_iters: u32,
        argon2_memory_mb: u32,
        argon2_parallelism: u32,
    ) -> Result<Self, validator::ValidationErrors>
    where
        A: ToString,
        B: ToString,
        C: ToString,
        D: ToString,
    {
        let res = User {
            uuid: uuid::Uuid::new_v4().into(),
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            salt,
            argon2_iters,
            argon2_memory_mb,
            argon2_parallelism,
            last_updated: None,
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
            &"a".repeat(255),
            &"b".repeat(255),
            rng::random_bytes(16),
            2,
            32,
            2,
        );

        assert!(res_ok.is_ok());

        let res_invalid_email = User::new(
            "hello.com",
            "123456",
            "a".repeat(255),
            "b".repeat(255),
            rng::random_bytes(16),
            2,
            32,
            2,
        );

        assert!(res_invalid_email.is_err());

        let res_invalid_email_err = res_invalid_email.unwrap_err();
        let res_invalid_email_err = res_invalid_email_err.field_errors();

        assert!(res_invalid_email_err.contains_key("email"));

        let res_fn_too_short = User::new(
            "hello@mail.com",
            "123456",
            "a".repeat(1),
            "b".repeat(255),
            rng::random_bytes(16),
            2,
            32,
            2,
        );

        assert!(res_fn_too_short.is_err());

        let res_fn_too_short_err = res_fn_too_short.unwrap_err();
        let res_fn_too_short_err = res_fn_too_short_err.field_errors();

        assert!(res_fn_too_short_err.contains_key("first_name"));

        let res_fn_too_long = User::new(
            "hello@mail.com",
            "123456",
            "a".repeat(256),
            "b".repeat(255),
            rng::random_bytes(16),
            2,
            32,
            2,
        );

        assert!(res_fn_too_long.is_err());

        let res_fn_too_long_err = res_fn_too_long.unwrap_err();
        let res_fn_too_long_err = res_fn_too_long_err.field_errors();

        assert!(res_fn_too_long_err.contains_key("first_name"));

        let res_ln_too_short = User::new(
            "hello@mail.com",
            "123456",
            "a".repeat(255),
            "b".repeat(1),
            rng::random_bytes(16),
            2,
            32,
            2,
        );

        assert!(res_ln_too_short.is_err());

        let res_ln_too_short_err = res_ln_too_short.unwrap_err();
        let res_ln_too_short_err = res_ln_too_short_err.field_errors();

        assert!(res_ln_too_short_err.contains_key("last_name"));

        let res_ln_too_long = User::new(
            "hello@mail.com",
            "123456",
            "a".repeat(255),
            "b".repeat(256),
            rng::random_bytes(16),
            2,
            32,
            2,
        );

        assert!(res_ln_too_long.is_err());

        let res_ln_too_long_err = res_ln_too_long.unwrap_err();
        let res_ln_too_long_err = res_ln_too_long_err.field_errors();

        assert!(res_ln_too_long_err.contains_key("last_name"));
    }
}
