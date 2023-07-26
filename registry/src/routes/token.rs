use crate::errors::ExtensionRegistryError;
use crate::token::generate_token;
use actix_web::HttpResponse;
use actix_web::{post, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub azp: String,
    pub exp: i32,
    pub iat: i32,
    pub iss: String,
    pub nbf: i32,
    pub sid: String,
    pub sub: String,
    pub userName: String,
}

#[post("/new")]
pub async fn new_token(
    conn: web::Data<Pool<Postgres>>,
    auth: BearerAuth,
) -> Result<HttpResponse, ExtensionRegistryError> {
    // Get JWT token from header
    let jwt = auth.token();
    // Decode JWT token
    let claims = decode_claims(jwt)?;
    // Generate API token for user
    let (token, token_sha) = generate_token();
    // Create a database transaction
    let mut tx = conn.begin().await?;
    // Save as record in DB with clerk user ID and username
    sqlx::query!(
        "
            INSERT INTO api_tokens(user_id, user_name, token, created_at)
            VALUES ($1, $2, $3, (now() at time zone 'utc'))
            ",
        claims.sub,
        claims.userName,
        token_sha
    )
    .execute(&mut tx)
    .await?;
    tx.commit().await?;
    Ok(HttpResponse::Ok().body(token))
}

fn b64_decode(b64_encoded: &str) -> Result<String, ExtensionRegistryError> {
    let bytes = general_purpose::URL_SAFE_NO_PAD
        .decode(b64_encoded)
        .map_err(|_| ExtensionRegistryError::TokenError("invalid base64".to_owned()))?;
    Ok(String::from_utf8(bytes)?)
}

pub fn decode_claims(jwt: &str) -> Result<Claims, ExtensionRegistryError> {
    let parts: Vec<&str> = jwt.split('.').collect();
    let payload = parts[1];
    let decoded_payload = b64_decode(payload)?;
    let claims: Claims = serde_json::from_str(&decoded_payload)?;
    Ok(claims)
}
