use sqlx::sqlite;

/// Trait for providing the database table name
pub trait TableName {
    const NAME: &'static str;
}

/// Pool provider
pub trait PoolProvider {
    fn get_pool(&self) -> &sqlite::SqlitePool;
}

// /// Create new database entry
// pub async fn create<T>(&self, table_name: &str, data_map: T) -> Result<T, crate::error::Error>
// where
//     T: serde::ser::Serialize + serde::de::DeserializeOwned,
//     T: for<'a> sqlx::FromRow<'a, sqlite::SqliteRow> + Unpin + Send,
// {
// }
//
// /// Read from database.
// pub async fn read() {}
//
// /// Update new database entry
// pub async fn update() {}
//
// /// Delete a database entry
// pub async fn delete() {}
