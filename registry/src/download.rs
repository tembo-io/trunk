//! Functionality for downloading extensions and maintaining download counts
use crate::errors::ExtensionRegistryError;
use actix_web::web;
use sqlx::{Pool, Postgres};

/// Find an extension's lastest version given it's extension_id
pub async fn latest_version(
    extension_id: i32,
    conn: web::Data<Pool<Postgres>>,
) -> Result<String, ExtensionRegistryError> {
    // Create a transaction on the database
    let mut tx = conn.begin().await?;
    let latest = sqlx::query!("SELECT num FROM versions WHERE extension_id = $1 ORDER BY string_to_array(num, '.')::int[] DESC LIMIT 1;", extension_id)
        .fetch_one(&mut tx)
        .await?;
    Ok(latest.num.unwrap())
}
