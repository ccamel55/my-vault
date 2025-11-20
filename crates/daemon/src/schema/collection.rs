use std::str::FromStr;
use validator::Validate;

/// Collection row entry
#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow, validator::Validate)]
pub struct Collection {
    #[serde(deserialize_with = "shared_core::serde::uuid::Hyphenated::deserialize")]
    pub uuid: uuid::fmt::Hyphenated,
    #[validate(length(min = 3, max = 255))]
    pub name: String,
}

impl Default for Collection {
    fn default() -> Self {
        Self {
            uuid: uuid::fmt::Hyphenated::from_str("0b1e5b28-d01a-4419-af8c-2d582170bc7e").unwrap(),
            name: "example-collection".to_string(),
        }
    }
}

impl Collection {
    pub fn new<A>(name: A) -> Result<Self, validator::ValidationErrors>
    where
        A: ToString,
    {
        let res = Collection {
            uuid: uuid::Uuid::new_v4().into(),
            name: name.to_string(),
        };

        res.validate()?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::Collection;

    #[tokio::test]
    async fn new_collection() {
        let res_ok = Collection::new("my-collection");

        assert!(res_ok.is_ok());

        let res_username_to_short = Collection::new("my");

        assert!(res_username_to_short.is_err());

        let res_invalid_username_err = res_username_to_short.unwrap_err();
        let res_invalid_username_err = res_invalid_username_err.field_errors();

        assert!(res_invalid_username_err.contains_key("name"));

        let res_username_to_long = Collection::new("a".repeat(256));

        assert!(res_username_to_long.is_err());

        let res_username_to_long = res_username_to_long.unwrap_err();
        let res_username_to_long = res_username_to_long.field_errors();

        assert!(res_username_to_long.contains_key("name"));
    }
}
