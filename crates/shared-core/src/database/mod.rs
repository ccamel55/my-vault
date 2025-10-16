mod crud;

use sqlx::sqlite;
use std::marker::PhantomData;
use std::str::FromStr;

pub use crud::*;

/// Trait for providing the database file name
pub trait DatabaseName {
    const NAME: &'static str;
}

#[derive(Debug)]
pub struct Database<D: DatabaseName> {
    sqlite_pool: sqlite::SqlitePool,
    database_name: PhantomData<D>,
}

impl<D: DatabaseName> Database<D> {
    /// Load database instance.
    /// This will create a new local database if none is found.
    pub async fn load() -> Result<Self, crate::error::Error> {
        let sqlite_file_path = format!(
            "sqlite://{}",
            crate::GLOBAL_CONFIG_PATH.join(D::NAME).display()
        );

        tracing::info!("daemon sqlite: {sqlite_file_path}");

        // Create a new SQLX connection to local database file.
        let options = sqlite::SqliteConnectOptions::from_str(&sqlite_file_path)?
            .read_only(false)
            .create_if_missing(true);

        Ok(Self {
            sqlite_pool: sqlite::SqlitePool::connect_with(options).await?,
            database_name: PhantomData,
        })
    }

    pub fn get_pool(&self) -> &sqlite::SqlitePool {
        &self.sqlite_pool
    }
}
