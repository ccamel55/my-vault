use shared_core::crypt;
use shared_core::database;

/// Holds information about current daemon client.
#[derive(Debug)]
pub struct DaemonClient {
    jwt: crypt::JwtFactory<Self>,
    time_start: chrono::DateTime<chrono::Utc>,
    database: database::Database<Self>,
}

impl crypt::JwtFactoryMetadata for DaemonClient {
    const RSA_PEM_PRIVATE: &'static str = "rsa.pem";

    const ISSUER: &'static str = "my-vault-service";
}

impl database::DatabaseName for DaemonClient {
    const NAME: &'static str = "daemon.sqlite3";
}

impl DaemonClient {
    /// Create an instance of the client.
    pub async fn start() -> anyhow::Result<Self> {
        let database = database::Database::load().await?;

        // Perform migration to ensure that our database is always upto date.
        sqlx::migrate!().run(database.get_pool()).await?;

        // TODO REMOVE ME RETARDS
        let email = format!("{}@gmail.com", uuid::Uuid::new_v4());
        crate::database::controller::ControllerUser::register(
            &database, email, "shit", "fuck", "you",
        )
        .await?;

        Ok(Self {
            jwt: crypt::JwtFactory::new().await?,
            time_start: chrono::Utc::now(),
            database,
        })
    }

    /// Get time daemon was started.
    pub fn get_time_started(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.time_start
    }

    /// Get current database.
    pub fn get_database(&self) -> &database::Database<Self> {
        &self.database
    }
}
