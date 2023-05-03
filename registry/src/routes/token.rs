use actix_web::HttpResponse;
use crate::errors::ExtensionRegistryError;
use actix_web::{post};

#[post("/new")]
pub async fn new_token() -> Result<HttpResponse, ExtensionRegistryError> {
    // Get JWT token from header
    // Decode JWT token
    // Generate API token for user
    // Save as record in DB with clerk user ID
    Ok(HttpResponse::Ok().body("token"))
}
