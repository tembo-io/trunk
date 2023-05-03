use crate::errors::ExtensionRegistryError;
use crate::token::generate_token;
use actix_web::HttpResponse;
use actix_web::{post, web};
use sqlx::{Pool, Postgres};

#[post("/new")]
pub async fn new_token(
    conn: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, ExtensionRegistryError> {
    // Get JWT token from header
    // Decode JWT token
    // Generate API token for user
    let (token, token_sha) = generate_token();
    // Create a database transaction
    let mut tx = conn.begin().await?;
    // Save as record in DB with clerk user ID
    sqlx::query!(
        "
            INSERT INTO api_tokens(token, created_at)
            VALUES ($1, (now() at time zone 'utc'))
            ",
        token_sha
    )
    .execute(&mut tx)
    .await?;
    tx.commit().await?;
    Ok(HttpResponse::Ok().body(token))
}
