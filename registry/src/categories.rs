use crate::errors::ExtensionRegistryError;
use actix_web::web::Data;
use log::{debug, info};
use sqlx::{Pool, Postgres};

// Update categories for a given extension
pub async fn update_extension_categories(
    extension_id: i64,
    categories: Vec<String>,
    conn: Data<Pool<Postgres>>,
) -> Result<(), ExtensionRegistryError> {
    debug!("categories: {:?}", categories);
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

    let existing_ids: Vec<i64> = existing.into_iter().map(|x| x.category_id as i64).collect();

    // If id not in existing, add
    for category_id in new_ids.clone() {
        if !existing_ids.contains(&category_id) {
            info!(
                "adding category_id {} to extension_id {}",
                category_id, extension_id
            );
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
            sqlx::query!(
                "
                UPDATE categories
                SET extension_count = extension_count + 1
                WHERE id = $1
                ",
                category_id as i32
            )
            .execute(&mut tx)
            .await?;
        }
    }
    // If existing not in ids, delete
    for category_id in existing_ids {
        if !new_ids.contains(&category_id) {
            info!(
                "removing category_id {} from extension_id {}",
                category_id, extension_id
            );
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
            sqlx::query!(
                "
                UPDATE categories
                SET extension_count = extension_count - 1
                WHERE id = $1
                ",
                category_id as i32
            )
            .execute(&mut tx)
            .await?;
        }
    }

    tx.commit().await?;
    Ok(())
}

// Get category IDs for a given vector of category names
pub async fn get_category_ids(
    categories: Vec<String>,
    conn: Data<Pool<Postgres>>,
) -> Result<Vec<i64>, ExtensionRegistryError> {
    let mut ids: Vec<i64> = Vec::new();
    for slug in categories {
        let id = sqlx::query!("SELECT id FROM categories WHERE slug = $1", slug)
            .fetch_optional(conn.as_ref())
            .await?;
        if id.is_some() {
            ids.push(id.unwrap().id)
        }
    }
    Ok(ids)
}

// Get category names for a given vector of category IDs
pub async fn get_category_names(
    category_ids: Vec<i64>,
    conn: Data<Pool<Postgres>>,
) -> Result<Vec<String>, ExtensionRegistryError> {
    let mut categories: Vec<String> = Vec::new();
    for id in category_ids {
        let name = sqlx::query!("SELECT name FROM categories WHERE id = $1", id)
            .fetch_one(conn.as_ref())
            .await?;
        categories.push(name.name)
    }
    Ok(categories)
}

// Get categories for a given extension
pub async fn get_categories_for_extension(
    extension_id: i64,
    conn: Data<Pool<Postgres>>,
) -> Result<Vec<String>, ExtensionRegistryError> {
    let category_ids = sqlx::query!(
        "SELECT category_id FROM extensions_categories WHERE extension_id = $1",
        extension_id as i32
    )
    .fetch_all(conn.as_ref())
    .await?;
    let category_ids: Vec<i64> = category_ids
        .into_iter()
        .map(|x| x.category_id as i64)
        .collect();
    let categories = get_category_names(category_ids, conn).await?;
    Ok(categories)
}
