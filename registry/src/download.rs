//! Functionality for downloading extensions and maintaining download counts
use crate::errors::ExtensionRegistryError;
use actix_web::web;
use sqlx::{Pool, Postgres};

pub async fn latest_version(
    extension_name: &str,
    conn: web::Data<Pool<Postgres>>,
) -> Result<String, ExtensionRegistryError> {
    // Create a transaction on the database
    let mut tx = conn.begin().await?;
    let ext = sqlx::query!("SELECT * FROM extensions WHERE name = $1", extension_name)
        .fetch_one(&mut tx)
        .await?;
    let id: i32 = ext.id as i32;
    let latest = sqlx::query!("SELECT * FROM versions WHERE extension_id = $1 ORDER BY string_to_array(num, '.')::int[] DESC LIMIT 1;", id)
        .fetch_one(&mut tx)
        .await?;
    Ok(latest.num.unwrap())
}
