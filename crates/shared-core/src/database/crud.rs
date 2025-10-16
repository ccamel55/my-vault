use sqlx::sqlite;
use std::collections::HashMap;

/// Trait for providing the database table name
pub trait TableName {
    const NAME: &'static str;
}

/// Create new database entry
pub async fn create<D, N, T>(
    database: super::Database<D>,
    data: T,
) -> Result<T, crate::error::Error>
where
    D: super::DatabaseName,
    N: TableName,
    T: serde::ser::Serialize + serde::de::DeserializeOwned,
    T: for<'a> sqlx::FromRow<'a, sqlite::SqliteRow> + Unpin + Send,
{
    // Get hash map with field names and values.
    let s = serde_json::to_string(&data).unwrap();
    let data_map: HashMap<String, serde_json::Value> = serde_json::from_str(&s).unwrap();

    let keys = data_map
        .keys()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let values = data_map
        .values()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let query = format!(
        "INSERT INTO {} ({}) VALUES ({}) RETURNING {}",
        N::NAME,
        &keys,
        &values,
        &keys
    );

    let result = sqlx::query_as(&query)
        .fetch_one(database.get_pool())
        .await
        .map_err(crate::error::Error::from)?;

    Ok(result)
}

/// Read from database.
pub async fn read() {}

/// Update new database entry
pub async fn update() {}

/// Delete a database entry
pub async fn delete() {}
