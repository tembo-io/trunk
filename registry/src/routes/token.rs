use crate::errors::ExtensionRegistryError;
use actix_web::post;
use actix_web::HttpResponse;

#[post("/new")]
pub async fn new_token() -> Result<HttpResponse, ExtensionRegistryError> {
    // Get JWT token from header
    // Decode JWT token
    // Generate API token for user
    // Save as record in DB with clerk user ID
    Ok(HttpResponse::Ok().body("token"))
}
