use sqlx::sqlite;
use std::collections::HashMap;

/// Trait for providing the database table name
pub trait TableName {
    const NAME: &'static str;
}

/// Create new database entry
pub async fn create<N, T>(database: &sqlite::SqlitePool, data: T) -> Result<T, crate::error::Error>
where
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

    let returning = crate::serde::struct_fields::<T>()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let query = format!(
        "INSERT INTO {} ({}) VALUES ({}) RETURNING {}",
        N::NAME,
        &keys,
        &values,
        &returning,
    );

    let result = sqlx::query_as(&query)
        .fetch_one(database)
        .await
        .map_err(crate::error::Error::from)?;

    Ok(result)
}

/// Read from database.
pub async fn read<N, T>(
    database: &sqlite::SqlitePool,
    where_map: Vec<(&'static str, String)>,
) -> Result<T, crate::error::Error>
where
    N: TableName,
    T: serde::ser::Serialize + serde::de::DeserializeOwned,
    T: for<'a> sqlx::FromRow<'a, sqlite::SqliteRow> + Unpin + Send,
{
    let returning = crate::serde::struct_fields::<T>()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let mut query: sqlx::QueryBuilder<'_, sqlx::Sqlite> =
        sqlx::query_builder::QueryBuilder::new(format!("SELECT {} FROM {}", returning, N::NAME));

    if !where_map.is_empty() {
        query.push(" WHERE");

        for (i, (k, v)) in where_map.iter().enumerate() {
            if i > 0 {
                query.push(" AND");
            }

            query.push(" ");
            query.push(k);
            query.push(" = ");
            query.push(format!("'{}'", v));
        }
    }

    query.push(" LIMIT 1");

    let result = sqlx::query_as(query.sql())
        .fetch_one(database)
        .await
        .map_err(crate::error::Error::from)?;

    Ok(result)
}

/// Update new database entry
pub async fn update<N, T>(
    database: &sqlite::SqlitePool,
    where_map: Vec<(&'static str, String)>,
    data: T,
) -> Result<T, crate::error::Error>
where
    N: TableName,
    T: serde::ser::Serialize + serde::de::DeserializeOwned,
    T: for<'a> sqlx::FromRow<'a, sqlite::SqliteRow> + Unpin + Send,
{
    // Get hash map with field names and values.
    let s = serde_json::to_string(&data).unwrap();
    let data_map: HashMap<String, serde_json::Value> = serde_json::from_str(&s).unwrap();

    let returning = crate::serde::struct_fields::<T>()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    if data_map.is_empty() {
        return Err(crate::error::Error::Sqlx("data type invalid".into()));
    }

    let mut query: sqlx::QueryBuilder<'_, sqlx::Sqlite> =
        sqlx::query_builder::QueryBuilder::new(format!("UPDATE {} SET", N::NAME));

    for (i, (k, v)) in data_map.iter().enumerate() {
        if i > 0 {
            query.push(",");
        }

        query.push(" ");
        query.push(k);
        query.push(" = ");
        query.push(format!("{}", v));
    }

    if !where_map.is_empty() {
        query.push(" WHERE");

        for (i, (k, v)) in where_map.iter().enumerate() {
            if i > 0 {
                query.push(" AND");
            }

            query.push(" ");
            query.push(k);
            query.push(" = ");
            query.push(format!("'{}'", v));
        }
    }

    query.push(" RETURNING ");
    query.push(returning);

    println!("{}", query.sql());

    let result = sqlx::query_as(query.sql())
        .fetch_one(database)
        .await
        .map_err(crate::error::Error::from)?;

    Ok(result)
}

/// Delete a database entry
pub async fn delete<N, T>(
    database: &sqlite::SqlitePool,
    where_map: Vec<(&'static str, String)>,
) -> Result<(), crate::error::Error>
where
    N: TableName,
{
    let mut query: sqlx::QueryBuilder<'_, sqlx::Sqlite> =
        sqlx::query_builder::QueryBuilder::new(format!("DELETE FROM {}", N::NAME));

    if !where_map.is_empty() {
        query.push(" WHERE");

        for (i, (k, v)) in where_map.iter().enumerate() {
            if i > 0 {
                query.push(" AND");
            }

            query.push(" ");
            query.push(k);
            query.push(" = ");
            query.push(format!("'{}'", v));
        }
    }

    println!("{}", query.sql());

    // If rows changed is not 0 then it means we modified (deleted a row) the table.
    let rows_affected = sqlx::query(query.sql())
        .execute(database)
        .await
        .map_err(crate::error::Error::from)?
        .rows_affected();

    if rows_affected == 0 {
        return Err(sqlx::error::Error::RowNotFound.into());
    }

    Ok(())
}

/// Checks if something exists
pub async fn exists<N>(
    database: &sqlite::SqlitePool,
    where_map: Vec<(&'static str, String)>,
) -> Result<bool, crate::error::Error>
where
    N: TableName,
{
    let mut select_query: sqlx::QueryBuilder<'_, sqlx::Sqlite> =
        sqlx::query_builder::QueryBuilder::new(format!("SELECT 1 FROM {}", N::NAME));

    if !where_map.is_empty() {
        select_query.push(" WHERE");

        for (i, (k, v)) in where_map.iter().enumerate() {
            if i > 0 {
                select_query.push(" AND");
            }

            select_query.push(" ");
            select_query.push(k);
            select_query.push(" = ");
            select_query.push(format!("'{}'", v));
        }
    }

    let query = format!("SELECT EXISTS({})", select_query.sql());

    let result = sqlx::query_scalar(&query)
        .fetch_one(database)
        .await
        .map_err(crate::error::Error::from)?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use sqlx::sqlite;

    pub struct TestDatabase;

    impl super::TableName for TestDatabase {
        const NAME: &'static str = "my_table";
    }

    /// Test row
    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
    pub struct TestRow {
        pub name: String,
        pub password: String,
    }

    #[sqlx::test]
    async fn read(pool: sqlite::SqlitePool) -> sqlx::Result<()> {
        let result_1 =
            super::read::<TestDatabase, TestRow>(&pool, vec![("name", "bob".into())]).await;

        let result_2 =
            super::read::<TestDatabase, TestRow>(&pool, vec![("name", "jeff".into())]).await;

        let result_3 =
            super::read::<TestDatabase, TestRow>(&pool, vec![("firstname", "larry".into())]).await;

        assert!(result_1.is_ok());
        assert!(result_2.is_err());
        assert!(result_3.is_err());

        let result_1 = result_1.unwrap();

        assert_eq!(result_1.name, "bob");
        assert_eq!(result_1.password, "123");

        Ok(())
    }

    #[sqlx::test]
    async fn create(pool: sqlite::SqlitePool) -> sqlx::Result<()> {
        let entry = TestRow {
            name: "jeffery".into(),
            password: "steveiscool123".into(),
        };

        // Try insert duplicate entry
        let result_1 = super::create::<TestDatabase, _>(&pool, entry.clone()).await;
        let result_2 = super::create::<TestDatabase, _>(&pool, entry.clone()).await;

        assert!(result_1.is_ok());
        assert!(result_2.is_err());

        let result_1 = result_1.unwrap();

        assert_eq!(result_1.name, "jeffery");
        assert_eq!(result_1.password, "steveiscool123");

        Ok(())
    }

    #[sqlx::test]
    async fn update(pool: sqlite::SqlitePool) -> sqlx::Result<()> {
        let data_1 = TestRow {
            name: "bob".into(),
            password: "bobthebuilder".into(),
        };

        let data_2 = TestRow {
            name: "dog".into(),
            password: "doglover".into(),
        };

        let result_1 =
            super::update::<TestDatabase, TestRow>(&pool, vec![("name", "bob".into())], data_1)
                .await;

        let result_2 =
            super::update::<TestDatabase, TestRow>(&pool, vec![("name", "dog".into())], data_2)
                .await;

        assert!(result_1.is_ok());
        assert!(result_2.is_err());

        let result_1 = result_1.unwrap();

        assert_eq!(result_1.name, "bob");
        assert_eq!(result_1.password, "bobthebuilder");

        Ok(())
    }

    #[sqlx::test]
    async fn delete(pool: sqlite::SqlitePool) -> sqlx::Result<()> {
        let result_1 =
            super::delete::<TestDatabase, TestRow>(&pool, vec![("name", "bob".into())]).await;

        let result_2 =
            super::delete::<TestDatabase, TestRow>(&pool, vec![("name", "jeffery".into())]).await;

        assert!(result_1.is_ok());
        assert!(result_2.is_err());

        Ok(())
    }

    #[sqlx::test]
    async fn exists(pool: sqlite::SqlitePool) -> sqlx::Result<()> {
        let result_1 = super::exists::<TestDatabase>(&pool, vec![("name", "bob".into())]).await;
        let result_2 = super::exists::<TestDatabase>(&pool, vec![("name", "jeff".into())]).await;
        let result_3 =
            super::exists::<TestDatabase>(&pool, vec![("firstname", "larry".into())]).await;

        assert!(result_1.is_ok());
        assert!(result_2.is_ok());
        assert!(result_3.is_err());

        let result_1 = result_1.unwrap();
        let result_2 = result_2.unwrap();

        assert_eq!(result_1, true);
        assert_eq!(result_2, false);

        Ok(())
    }
}
