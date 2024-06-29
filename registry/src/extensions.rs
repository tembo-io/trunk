use crate::errors::ExtensionRegistryError;
use actix_web::web::Data;
use serde_json::{json, Value};
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
    user_id: &String,
    user_name: &String,
    conn: Data<Pool<Postgres>>,
) -> Result<(), ExtensionRegistryError> {
    let mut tx = conn.begin().await?;
    sqlx::query!(
        "
        INSERT INTO extension_owners(extension_id, owner_id, user_name, created_at, created_by)
        VALUES ($1, $2, $3, (now() at time zone 'utc'), $2)
        ",
        extension_id as i32,
        user_id,
        user_name,
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn extension_owners(
    extension_id: i32,
    conn: Data<Pool<Postgres>>,
) -> Result<Vec<Value>, ExtensionRegistryError> {
    let mut extension_owners: Vec<Value> = Vec::new();

    let owners = sqlx::query!(
        "SELECT * FROM extension_owners WHERE extension_id = $1;",
        extension_id
    )
    .fetch_all(conn.as_ref())
    .await?;

    for row in owners.into_iter() {
        let owner_id = row.owner_id;
        let user_name = row.user_name;
        let data = json!(
        {
          "userId": owner_id,
          "userName": user_name
        });
        extension_owners.push(data);
    }
    Ok(extension_owners)
}

pub async fn latest_license(
    extension_id: i32,
    latest_version: &str,
    conn: Data<Pool<Postgres>>,
) -> Result<String, ExtensionRegistryError> {
    // Create a transaction on the database
    let mut tx = conn.begin().await?;
    // Get latest version for extension
    let latest_license = sqlx::query!(
        "SELECT license FROM versions WHERE extension_id = $1 AND num = $2;",
        extension_id,
        latest_version
    )
    .fetch_one(&mut *tx)
    .await?;
    Ok(latest_license.license.unwrap_or("".to_string()))
}

pub async fn get_extension_id(
    extension_name: &str,
    conn: &Pool<Postgres>,
) -> Result<i64, ExtensionRegistryError> {
    let id = sqlx::query!("SELECT id FROM extensions WHERE name = $1", extension_name)
        .fetch_one(conn)
        .await?;

    Ok(id.id)
}
