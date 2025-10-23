mod crud;

use sqlx::sqlite;
use std::path::Path;
use std::str::FromStr;

pub use crud::*;

#[derive(Debug)]
pub struct Database {
    sqlite_pool: sqlite::SqlitePool,
}

impl Database {
    /// Load database instance.
    /// This will create a new local database if none is found.
    pub async fn load(
        sqlite_file_path: &Path,
        encryption_key: String,
    ) -> Result<Self, crate::error::Error> {
        let sqlite_file_path = format!("sqlite://{}", sqlite_file_path.display());

        tracing::info!("daemon sqlite: {sqlite_file_path}");

        // Create a new SQLX connection to local database file.
        let options = sqlite::SqliteConnectOptions::from_str(&sqlite_file_path)?
            .pragma("key", encryption_key)
            .read_only(false)
            .create_if_missing(true);

        Ok(Self {
            sqlite_pool: sqlite::SqlitePool::connect_with(options).await?,
        })
    }

    pub fn get_pool(&self) -> &sqlite::SqlitePool {
        &self.sqlite_pool
    }
}
