use crate::database::view;

use shared_core::database;

/// User table controller
pub struct ControllerUser;

impl database::TableName for ControllerUser {
    const NAME: &'static str = "users";
}

impl ControllerUser {
    /// Register a new user
    pub async fn register<A, B, C, D>(
        database: &database::Database,
        email: A,
        password: B,
        first_name: C,
        last_name: D,
    ) -> anyhow::Result<uuid::Uuid>
    where
        A: ToString,
        B: ToString,
        C: ToString,
        D: ToString,
    {
        // TODO Convert password into hashed blob that we store in database.

        let data = view::User::new(email, password.to_string(), first_name, last_name)?;
        let result = database::create::<Self, _>(database.get_pool(), data).await?;

        Ok(result.uuid.into_uuid())
    }
}
