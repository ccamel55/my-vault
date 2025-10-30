use crate::schema;
use shared_core::{database, error};
use sqlx::sqlite;

pub struct ModelUser;

impl database::TableName for ModelUser {
    const NAME: &'static str = "users_active";
}

impl ModelUser {
    /// Checks if user exists.
    pub async fn does_user_exist(
        pool: &sqlite::SqlitePool,
        username: String,
    ) -> Result<bool, error::Error> {
        let filter = vec![("username", username)];
        database::exists::<Self>(pool, filter).await
    }

    /// Get user from username.
    pub async fn get_user_from_username(
        pool: &sqlite::SqlitePool,
        username: String,
    ) -> Result<schema::User, error::Error> {
        let filter = vec![("username", username)];
        database::read::<Self, schema::User>(pool, filter).await
    }

    /// Get user from uuid.
    pub async fn get_user_from_uuid(
        pool: &sqlite::SqlitePool,
        uuid: uuid::Uuid,
    ) -> Result<schema::User, error::Error> {
        // Database stores uuid in hyphenated form.
        let filter = vec![("uuid", uuid.as_hyphenated().to_string())];
        database::read::<Self, schema::User>(pool, filter).await
    }

    /// Add a new user.
    pub async fn add_user(
        pool: &sqlite::SqlitePool,
        user: schema::User,
    ) -> Result<schema::User, error::Error> {
        // We get return data from database since on insert, things
        // like timestamps and other default values will be written.
        database::create::<Self, schema::User>(pool, user).await
    }
}

#[cfg(test)]
mod tests {
    use crate::model::ModelUser;

    use sqlx::sqlite;

    #[sqlx::test]
    async fn does_user_exist(pool: sqlite::SqlitePool) -> sqlx::Result<()> {
        // Check non-existing user
        let result_1 = ModelUser::does_user_exist(&pool, "jeff".into()).await;

        assert!(result_1.is_ok());
        assert_eq!(result_1.unwrap(), false);

        Ok(())
    }
}
