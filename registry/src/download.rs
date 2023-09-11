//! Functionality for downloading extensions and maintaining download counts
use sqlx::{Pool, Postgres};

/// Find an extension's lastest version given it's extension_id
pub async fn latest_version(
    extension_id: i32,
    conn: &Pool<Postgres>,
) -> Result<String, sqlx::Error> {
    let latest = sqlx::query!("SELECT num FROM versions WHERE extension_id = $1 ORDER BY string_to_array(num, '.')::int[] DESC LIMIT 1;", extension_id)
        .fetch_one(conn)
        .await?;
    Ok(latest.num.unwrap())
}
