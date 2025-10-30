use validator::Validate;

/// Source type
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum SourceType {
    Unknown = 0,
    Csv = 1,
}

impl From<u32> for SourceType {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::Csv,
            _ => Self::Unknown,
        }
    }
}

/// Source auth type
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum SourceAuthType {
    None = 0,
    Cipher = 1,
}

impl From<u32> for SourceAuthType {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::Cipher,
            _ => Self::None,
        }
    }
}

/// Secret source row entry
#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow, validator::Validate)]
pub struct Source {
    #[serde(deserialize_with = "shared_core::serde::uuid::Hyphenated::deserialize")]
    pub uuid: uuid::fmt::Hyphenated,
    #[validate(length(min = 3, max = 255))]
    pub name: String,
    pub description: Option<String>,
    pub source_type: u32,
    pub source_auth: Option<String>,
    pub source_auth_type: u32,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Source {
    pub fn new<A>(
        name: A,
        description: Option<String>,
        source_type: u32,
        source_auth: Option<String>,
        source_auth_type: u32,
    ) -> Result<Self, validator::ValidationErrors>
    where
        A: ToString,
    {
        let res = Source {
            uuid: uuid::Uuid::new_v4().into(),
            name: name.to_string(),
            description,
            source_type,
            source_auth,
            source_auth_type,
            created_at: None,
            updated_at: None,
        };

        res.validate()?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::{Source, SourceAuthType, SourceType};

    #[tokio::test]
    async fn new_source() {
        let res_ok = Source::new(
            "bob",
            Some("information from bob".to_string()),
            SourceType::Csv as u32,
            None,
            SourceAuthType::None as u32,
        );

        assert!(res_ok.is_ok());

        let res_username_to_short = Source::new(
            "b",
            Some("information from bob".to_string()),
            SourceType::Csv as u32,
            None,
            SourceAuthType::None as u32,
        );

        assert!(res_username_to_short.is_err());

        let res_username_to_short = res_username_to_short.unwrap_err();
        let res_username_to_short = res_username_to_short.field_errors();

        assert!(res_username_to_short.contains_key("name"));

        let res_username_to_long = Source::new(
            "b".repeat(256),
            Some("information from bob".to_string()),
            SourceType::Csv as u32,
            None,
            SourceAuthType::None as u32,
        );

        assert!(res_username_to_long.is_err());

        let res_username_to_long = res_username_to_long.unwrap_err();
        let res_username_to_long = res_username_to_long.field_errors();

        assert!(res_username_to_long.contains_key("name"));
    }
}
