#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error - ${0}")]
    IO(std::io::Error),

    #[error("config error - ${0}")]
    Config(String),

    #[error("error creating log file - ${0}")]
    LogFile(String),

    #[error("could not install tracing subscriber - ${0}")]
    TracingSubscriber(String),

    #[error("{0}")]
    Sqlx(String),

    #[error("error from migration - ${0}")]
    Migration(#[source] Box<sqlx::migrate::MigrateError>),

    #[error("error from database - ${0}")]
    Database(#[source] Box<dyn sqlx::error::DatabaseError>),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(value: toml::ser::Error) -> Self {
        Error::Config(value.to_string())
    }
}

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Error::Config(value.to_string())
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::error::Error::Database(e) => Error::Database(e),
            sqlx::error::Error::Migrate(e) => Error::Migration(e),
            e => Error::Sqlx(e.to_string()),
        }
    }
}
