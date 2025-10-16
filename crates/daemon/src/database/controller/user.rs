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
        password_hash: C,
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
        let data = view::User::new(email, password_hash, first_name, last_name)?;
        let result = database::create::<Self, _, _>(database, data).await?;

        Ok(result.uuid)
    }
}
