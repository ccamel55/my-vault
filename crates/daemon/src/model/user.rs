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
    use crate::schema::User;

    use sqlx::sqlite;

    #[sqlx::test]
    async fn does_user_exist(pool: sqlite::SqlitePool) -> sqlx::Result<()> {
        // Check non-existing user
        let result_1 = ModelUser::does_user_exist(&pool, "jeff".into()).await;

        assert!(result_1.is_ok());
        assert_eq!(result_1.unwrap(), false);

        // Add a user and then check again.
        let user = User {
            username: "jeff".into(),
            ..User::default()
        };
        let result_add = ModelUser::add_user(&pool, user).await;

        assert!(result_add.is_ok());

        // Should exist now
        let result_2 = ModelUser::does_user_exist(&pool, "jeff".into()).await;

        assert!(result_2.is_ok());
        assert_eq!(result_2.unwrap(), true);

        // Make sure if we query a different user it doesn't exist
        let result_3 = ModelUser::does_user_exist(&pool, "bob".into()).await;

        assert!(result_3.is_ok());
        assert_eq!(result_3.unwrap(), false);

        Ok(())
    }

    #[sqlx::test]
    async fn get_user_from_username(pool: sqlite::SqlitePool) -> sqlx::Result<()> {
        let result_1 = ModelUser::get_user_from_username(&pool, "jeff".into()).await;

        assert!(result_1.is_err());

        let user = User {
            username: "jeff".into(),
            ..User::default()
        };
        let result_add = ModelUser::add_user(&pool, user).await;

        assert!(result_add.is_ok());

        let result_2 = ModelUser::get_user_from_username(&pool, result_add.unwrap().username).await;

        assert!(result_2.is_ok());

        Ok(())
    }

    #[sqlx::test]
    async fn get_user_from_uuid(pool: sqlite::SqlitePool) -> sqlx::Result<()> {
        let result_1 = ModelUser::get_user_from_username(&pool, "jeff".into()).await;

        assert!(result_1.is_err());

        let user = User {
            username: "jeff".into(),
            ..User::default()
        };
        let result_add = ModelUser::add_user(&pool, user).await;

        assert!(result_add.is_ok());

        let result_2 =
            ModelUser::get_user_from_uuid(&pool, result_add.unwrap().uuid.into_uuid()).await;

        assert!(result_2.is_ok());

        Ok(())
    }
}
