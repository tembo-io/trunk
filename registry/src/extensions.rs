use crate::errors::ExtensionRegistryError;
use actix_web::web::Data;
use sqlx::{Pool, Postgres};

pub fn check_input(input: &str) -> Result<(), ExtensionRegistryError> {
    let valid = input
        .as_bytes()
        .iter()
        .all(|&c| c.is_ascii_alphanumeric() || c == b'_');
    match valid {
        true => Ok(()),
        false => Err(ExtensionRegistryError::ResponseError()),
    }
}

pub async fn add_extension_owner(
    extension_id: i64,
    user_id: String,
    conn: Data<Pool<Postgres>>,
) -> Result<(), ExtensionRegistryError> {
    let mut tx = conn.begin().await?;
    sqlx::query!(
        "
        INSERT INTO extension_owners(extension_id, owner_id, created_at, created_by)
        VALUES ($1, $2, (now() at time zone 'utc'), $3)
        ",
        extension_id as i32,
        user_id,
        user_id
    )
    .execute(&mut tx)
    .await?;
    tx.commit().await?;
    Ok(())
}
