mod crud;

use sqlx::sqlite;

pub use crud::*;

/// Trait for providing the database table name
pub trait TableName {
    const NAME: &'static str;
}

/// Pool provider
pub trait PoolProvider {
    fn get_pool(&self) -> &sqlite::SqlitePool;
}
