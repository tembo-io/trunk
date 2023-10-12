//! Functionality for downloading extensions and maintaining download counts
use crate::download::{check_version, latest_version};
use crate::errors::Result;
use crate::uploader::extension_location;
use crate::{config::Config, extensions::get_extension_id};
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
) -> Result<HttpResponse> {
    let (name, mut version) = path.into_inner();
    let extension_id = get_extension_id(&name, conn.as_ref()).await?;

    // Use latest version if 'latest' provided as version
    if version == "latest" {
        version = latest_version(extension_id as _, conn.as_ref()).await?;
    } else {
        let version_exists = check_version(conn.as_ref(), extension_id as _, &version).await?;
        if !version_exists {
            return Ok(HttpResponse::NotFound().body("Version not found"));
        }
    }
    let url = extension_location(&cfg.bucket_name, &name, &version);
    info!(
        "Download requested for {} version {}. URL: {}",
        name, version, url
    );
    increase_download_count(conn.as_ref(), extension_id as _).await?;

    Ok(HttpResponse::Ok().body(url))
}

async fn increase_download_count(pool: &Pool<Postgres>, extension_id: i32) -> Result {
    sqlx::query!(
        "UPDATE versions
        SET download_count = download_count + 1
        WHERE extension_id = $1",
        extension_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
