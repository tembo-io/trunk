use crate::download::latest_version;
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
    .execute(&mut tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn extension_owners(
    extension_name: &str,
    conn: Data<Pool<Postgres>>,
) -> Result<Vec<Value>, ExtensionRegistryError> {
    let mut extension_owners: Vec<Value> = Vec::new();
    // Create a transaction on the database
    let mut tx = conn.begin().await?;
    let ext = sqlx::query!("SELECT id FROM extensions WHERE name = $1", extension_name)
        .fetch_one(&mut tx)
        .await?;
    let id: i32 = ext.id as i32;
    let owners = sqlx::query!(
        "SELECT * FROM extension_owners WHERE extension_id = $1;",
        id
    )
    .fetch_all(&mut tx)
    .await?;
    for row in owners.iter() {
        let owner_id = row.owner_id.to_owned();
        let user_name = row.user_name.to_owned();
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
    extension_name: &str,
    conn: Data<Pool<Postgres>>,
) -> Result<String, ExtensionRegistryError> {
    // Get latest version for extension
    let latest_version = latest_version(extension_name, conn.clone()).await?;
    // Create a transaction on the database
    let mut tx = conn.begin().await?;
    let ext = sqlx::query!("SELECT id FROM extensions WHERE name = $1", extension_name)
        .fetch_one(&mut tx)
        .await?;
    let id: i32 = ext.id as i32;
    let latest_license = sqlx::query!(
        "SELECT license FROM versions WHERE extension_id = $1 AND num = $2;",
        id,
        latest_version
    )
    .fetch_one(&mut tx)
    .await?;
    Ok(latest_license.license.unwrap_or("".to_string()))
}

pub async fn get_extension_id(
    extension_name: String,
    conn: Data<Pool<Postgres>>,
) -> Result<i64, ExtensionRegistryError> {
    let mut tx = conn.begin().await?;
    let id = sqlx::query!("SELECT id FROM extensions WHERE name = $1", extension_name)
        .fetch_one(&mut tx)
        .await?;
    Ok(id.id)
}

// Update categories for a given extension
pub async fn update_extension_categories(
    extension_id: i64,
    categories: Vec<String>,
    conn: Data<Pool<Postgres>>,
) -> Result<(), ExtensionRegistryError> {
    let mut tx = conn.begin().await?;
    // Get category IDs
    let new_ids = get_category_ids(categories, conn).await?;
    // Get existing categories for the extension
    let existing = sqlx::query!(
        "SELECT category_id FROM extensions_categories WHERE extension_id = $1;",
        extension_id as i32
    )
    .fetch_all(&mut tx)
    .await?;

    let existing_ids: Vec<i64> = existing
        .into_iter()
        .map(|x| x.category_id.unwrap() as i64)
        .collect();

    // If id not in existing, add
    for category_id in new_ids.clone() {
        if !existing_ids.contains(&category_id) {
            sqlx::query!(
                "
                INSERT INTO extensions_categories(extension_id, category_id)
                VALUES ($1, $2)
                ",
                extension_id as i32,
                category_id as i32
            )
            .execute(&mut tx)
            .await?;
        }
    }
    // If existing not in ids, delete
    for category_id in existing_ids {
        if !new_ids.contains(&category_id) {
            sqlx::query!(
                "
                DELETE FROM extensions_categories
                WHERE extension_id = $1
                AND category_id = $2
                ",
                extension_id as i32,
                category_id as i32
            )
            .execute(&mut tx)
            .await?;
        }
    }

    tx.commit().await?;
    Ok(())
}

pub async fn get_category_ids(
    categories: Vec<String>,
    conn: Data<Pool<Postgres>>,
) -> Result<Vec<i64>, ExtensionRegistryError> {
    let mut ids: Vec<i64> = Vec::new();
    let mut tx = conn.begin().await?;
    for slug in categories {
        let id = sqlx::query!("SELECT id FROM categories WHERE slug = $1", slug)
            .fetch_one(&mut tx)
            .await?;
        ids.push(id.id)
    }
    Ok(ids)
}

pub async fn get_category_names(
    category_ids: Vec<i64>,
    conn: Data<Pool<Postgres>>,
) -> Result<Vec<String>, ExtensionRegistryError> {
    let mut categories: Vec<String> = Vec::new();
    let mut tx = conn.begin().await?;
    for id in category_ids {
        let name = sqlx::query!("SELECT name FROM categories WHERE id = $1", id)
            .fetch_one(&mut tx)
            .await?;
        categories.push(name.name.unwrap())
    }
    Ok(categories)
}

pub async fn get_categories(
    extension_id: i64,
    conn: Data<Pool<Postgres>>,
) -> Result<Vec<String>, ExtensionRegistryError> {
    // let mut category_ids: Vec<i64> = Vec::new();
    let mut categories: Vec<String> = Vec::new();
    let mut tx = conn.begin().await?;
    let category_ids = sqlx::query!(
        "SELECT category_id FROM extensions_categories WHERE extension_id = $1",
        extension_id as i32
    )
    .fetch_all(&mut tx)
    .await?;
    let category_ids: Vec<i64> = category_ids
        .into_iter()
        .map(|x| x.category_id.unwrap() as i64)
        .collect();
    for id in category_ids {
        let name = sqlx::query!("SELECT name FROM categories WHERE id = $1", id as i32)
            .fetch_one(&mut tx)
            .await?;
        categories.push(name.name.unwrap())
    }
    Ok(categories)
}
