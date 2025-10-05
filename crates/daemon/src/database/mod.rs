pub mod model;

use sqlx::sqlite;
use std::str::FromStr;

/// Name of database file for daemon.
const DAEMON_DATABASE_FILE_NAME: &str = "daemon.sqlite";

#[derive(Debug)]
pub struct Database {
    sqlite_pool: sqlite::SqlitePool,
}

impl Database {
    /// Load database instance.
    /// This will create a new local database if none is found.
    pub async fn load() -> anyhow::Result<Self> {
        let global_database_path = shared_core::GLOBAL_CONFIG_PATH.to_path_buf();
        let global_database_path_exists =
            global_database_path.exists() && global_database_path.is_dir();

        tracing::debug!("database folder: {}", global_database_path.display());
        tracing::debug!("creating database folder: {}", !global_database_path_exists);

        // Make sure that our database folder exists
        if !global_database_path_exists {
            tokio::fs::create_dir(&global_database_path).await?;
        }

        let sqlite_file_path = format!(
            "sqlite://{}",
            global_database_path
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

        // let _ = sqlx::query(
        //     "INSERT INTO users (uuid, name, email, password_hash) VALUES ($1, $2, $3, $4);",
        // )
        // .bind("jeff".as_bytes())
        // .bind("jeff")
        // .bind("jeffcool@gmail.com")
        // .bind("jeff".as_bytes())
        // .execute(&sqlite_pool)
        // .await?;

        Ok(Self { sqlite_pool })
    }
}
