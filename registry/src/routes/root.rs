use actix_web::{get, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::errors::ExtensionRegistryError;

#[get("/")]
pub async fn ok() -> impl Responder {
    HttpResponse::Ok().json("ok")
}

#[tracing::instrument]
#[get("/")]
pub async fn auth_ok(auth: BearerAuth) -> Result<HttpResponse, ExtensionRegistryError> {
    Ok(HttpResponse::Ok().json("OK"))
}
