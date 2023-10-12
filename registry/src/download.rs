//! Functionality for downloading extensions and maintaining download counts
use crate::errors::ExtensionRegistryError;
use sqlx::{Pool, Postgres};

/// Find an extension's lastest version given it's extension_id
pub async fn latest_version(
    extension_id: i32,
    conn: &Pool<Postgres>,
) -> Result<String, ExtensionRegistryError> {
    // Create a transaction on the database
    let mut tx = conn.begin().await?;
    let latest = sqlx::query!("SELECT num FROM versions WHERE extension_id = $1 ORDER BY string_to_array(num, '.')::int[] DESC LIMIT 1;", extension_id)
        .fetch_one(&mut tx)
        .await?;
    Ok(latest.num.unwrap())
}

pub async fn check_version(
    pool: &Pool<Postgres>,
    extension_id: i32,
    version: &str,
) -> Result<bool, ExtensionRegistryError> {
    let record = sqlx::query!(
        "SELECT COUNT(*) > 0 AS version_exists
        FROM versions
        WHERE extension_id = $1
        AND num = $2",
        extension_id,
        version
    )
    .fetch_one(pool)
    .await?;

    Ok(record.version_exists.unwrap_or(false))
}
