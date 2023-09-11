use actix_web::web::Json;
use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::Serialize;

use crate::cache::Cache;
use crate::errors::ExtensionRegistryError;

#[get("/")]
pub async fn ok() -> impl Responder {
    HttpResponse::Ok().json("ok")
}

#[tracing::instrument(skip(_auth))]
#[get("/")]
pub async fn auth_ok(_auth: BearerAuth) -> Result<HttpResponse, ExtensionRegistryError> {
    Ok(HttpResponse::Ok().json("OK"))
}

#[derive(Serialize)]
pub struct ContentCleared {
    old_contents: Option<String>,
}

#[tracing::instrument(skip(_auth, cache))]
#[post("/cache/clear")]
pub async fn clear_extensions_cache(
    _auth: BearerAuth,
    cache: web::Data<Cache<String>>,
) -> Result<Json<ContentCleared>, ExtensionRegistryError> {
    let old_contents = cache.inner.write().await.take();
    let content_cleared = ContentCleared { old_contents };

    Ok(Json(content_cleared))
}
