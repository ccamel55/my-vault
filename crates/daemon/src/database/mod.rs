use sqlx::sqlite;
use std::str::FromStr;

/// Name of database file for daemon.
const DAEMON_DATABASE_FILE_NAME: &str = "daemon.sqlite3";

#[derive(Debug)]
pub struct Database {
    sqlite_pool: sqlite::SqlitePool,
}

impl Database {
    /// Load database instance.
    /// This will create a new local database if none is found.
    pub async fn load() -> anyhow::Result<Self> {
        let sqlite_file_path = format!(
            "sqlite://{}",
            shared_core::GLOBAL_CONFIG_PATH
                .join(DAEMON_DATABASE_FILE_NAME)
                .display()
        );

        tracing::info!("daemon sqlite: {sqlite_file_path}");

        // Create a new SQLX connection to local database file.
        let options = sqlite::SqliteConnectOptions::from_str(&sqlite_file_path)?
            .read_only(false)
            .create_if_missing(true);

        let sqlite_pool = sqlite::SqlitePool::connect_with(options).await?;

        // Perform migration to ensure that our database is always upto date.
        sqlx::migrate!().run(&sqlite_pool).await?;

        let _ = sqlx::query(
            "INSERT INTO users (uuid, email, password_hash, first_name, last_name) VALUES ($1, $2, $3, $4, $5);",
        )
            .bind("jeff".as_bytes())
            .bind("jeffcool@gmail.com")
            .bind("jeff")
            .bind("jeff".as_bytes())
            .bind("fat".as_bytes())
            .execute(&sqlite_pool)
            .await?;

        Ok(Self { sqlite_pool })
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
}
