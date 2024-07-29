use crate::errors::ExtensionRegistryError;
use actix_web::{get, web, HttpResponse};
use serde_json::{json, Value};
use sqlx::{Pool, Postgres};

#[get("/categories/all")]
pub async fn get_all_categories(
    conn: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, ExtensionRegistryError> {
    // Create a database transaction
    let mut rows = sqlx::query!("SELECT * FROM categories")
        .fetch_all(conn.as_ref())
        .await?;
    let mut categories: Vec<Value> = Vec::with_capacity(rows.len());

    // Make sure Featured is the first category to be sent
    {
        if let Some(featured_idx) = rows.iter().position(|row| row.name == "Featured") {
            rows.swap(0, featured_idx);
        }
    }

    for row in rows {
        let data = json!(
        {
          "name": row.name,
          "id": row.id,
          "slug": row.slug,
          "description": row.description,
          "extension_count": row.extension_count
        });
        categories.push(data);
    }

    // Return results in response
    let json = serde_json::to_string_pretty(&categories)?;
    Ok(HttpResponse::Ok().body(json))
}
