//! Functionality for downloading extensions and maintaining download counts
use crate::config::Config;
use crate::download::latest_version;
use crate::errors::ExtensionRegistryError;
use crate::uploader::extension_location;
use actix_web::{get, web, HttpResponse};
use sqlx::{Pool, Postgres};
use tracing::info;

/// Handles the `GET /extensions/:extension_name/:version/download` route.
/// This returns a URL to the location where the extension is stored.
#[get("/extensions/{extension_name}/{version}/download")]
pub async fn download(
    conn: web::Data<Pool<Postgres>>,
    cfg: web::Data<Config>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, ExtensionRegistryError> {
    let (name, mut version) = path.into_inner();
    // TODO(ianstanton) Increment download count for extension
    // Use latest version if 'latest' provided as version
    if version == "latest" {
        version = latest_version(&name, conn).await?;
    }
    let url = extension_location(&cfg.bucket_name, &name, &version);
    info!("Download requested for {} version {}", name, version);
    info!("URL: {}", url);
    Ok(HttpResponse::Ok().body(url))
}
