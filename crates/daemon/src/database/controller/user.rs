use crate::database::view;

use shared_core::database;

/// User table controller
pub struct ControllerUser;

impl database::TableName for ControllerUser {
    const NAME: &'static str = "users";
}

impl ControllerUser {
    /// Register a new user
    pub async fn register<A, B, C, D, E>(
        database: &database::Database<A>,
        email: B,
        password: C,
        first_name: D,
        last_name: E,
    ) -> anyhow::Result<uuid::Uuid>
    where
        A: database::DatabaseName,
        B: ToString,
        C: ToString,
        D: ToString,
        E: ToString,
    {
        // TODO Convert password into hashed blob that we store in database.

        let data = view::User::new(email, password.to_string(), first_name, last_name)?;
        let result = database::create::<Self, _>(database.get_pool(), data).await?;

        Ok(result.uuid.into_uuid())
    }
}
