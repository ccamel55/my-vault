use validator::Validate;

/// Secret type
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum SecretType {
    Unknown = 0,
    Cipher = 1,
    Key = 2,
}

impl From<u32> for SecretType {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::Cipher,
            2 => Self::Key,
            _ => Self::Unknown,
        }
    }
}

/// Secret row entry
#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow, validator::Validate)]
pub struct Secret {
    #[serde(deserialize_with = "shared_core::serde::uuid::Hyphenated::deserialize")]
    pub uuid: uuid::fmt::Hyphenated,
    #[validate(length(min = 3, max = 255))]
    pub name: String,
    pub key: Option<String>,
    pub description: Option<String>,
    pub secret: String,
    pub secret_type: u32,
}

impl Secret {
    pub fn new<A, B>(
        name: A,
        key: Option<String>,
        description: Option<String>,
        secret: B,
        secret_type: u32,
    ) -> Result<Self, validator::ValidationErrors>
    where
        A: ToString,
        B: ToString,
    {
        let res = Secret {
            uuid: uuid::Uuid::new_v4().into(),
            name: name.to_string(),
            key,
            description,
            secret: secret.to_string(),
            secret_type,
        };

        res.validate()?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::{Secret, SecretType};

    #[tokio::test]
    async fn new_secret() {
        let res_ok = Secret::new(
            "my-password",
            None,
            None,
            "super-secret-password",
            SecretType::Cipher as u32,
        );

        assert!(res_ok.is_ok());

        let res_username_to_short = Secret::new(
            "aa",
            None,
            None,
            "super-secret-password",
            SecretType::Cipher as u32,
        );

        assert!(res_username_to_short.is_err());

        let res_username_to_short = res_username_to_short.unwrap_err();
        let res_username_to_short = res_username_to_short.field_errors();

        assert!(res_username_to_short.contains_key("name"));

        let res_username_to_long = Secret::new(
            "a".repeat(256),
            None,
            None,
            "super-secret-password",
            SecretType::Cipher as u32,
        );

        assert!(res_username_to_long.is_err());

        let res_username_to_long = res_username_to_long.unwrap_err();
        let res_username_to_long = res_username_to_long.field_errors();

        assert!(res_username_to_long.contains_key("name"));
    }
}
