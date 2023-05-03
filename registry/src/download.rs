//! Functionality for downloading extensions and maintaining download counts
use crate::config::Config;
use crate::errors::ExtensionRegistryError;
use crate::uploader::extension_location;
use actix_web::{get, web, HttpResponse};
use log::info;
use sqlx::{Pool, Postgres};

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

pub async fn latest_version(
    extension_name: &str,
    conn: web::Data<Pool<Postgres>>,
) -> Result<String, ExtensionRegistryError> {
    // Create a transaction on the database, if there are no errors,
    // commit the transactions to record a new or updated extension.
    let mut tx = conn.begin().await?;
    let ext = sqlx::query!("SELECT * FROM extensions WHERE name = $1", extension_name)
        .fetch_one(&mut tx)
        .await?;
    let id: i32 = ext.id as i32;
    let latest = sqlx::query!("SELECT MAX(num) FROM versions WHERE extension_id = $1;", id)
        .fetch_one(&mut tx)
        .await?;
    Ok(latest.max.unwrap())
}
