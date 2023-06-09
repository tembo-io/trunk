use crate::errors::ExtensionRegistryError;
use actix_web::{get, web, HttpResponse};
use serde_json::{json, Value};
use sqlx::{Pool, Postgres};

#[get("/categories/all")]
pub async fn get_all_categories(
    conn: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, ExtensionRegistryError> {
    let mut categories: Vec<Value> = Vec::new();

    // Create a database transaction
    let mut tx = conn.begin().await?;
    let rows = sqlx::query!("SELECT * FROM categories")
        .fetch_all(&mut tx)
        .await?;
    for row in rows.iter() {
        let data = json!(
        {
          "name": row.name.to_owned(),
          "id": row.id,
          "slug": row.slug.to_owned(),
          "description": row.description.to_owned(),
          "extension_count": row.extension_count
        });
        categories.push(data);
    }
    // Return results in response
    let json = serde_json::to_string_pretty(&categories)?;
    Ok(HttpResponse::Ok().body(json))
}
