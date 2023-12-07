//! Functionality for downloading extensions and maintaining download counts
use std::borrow::Cow;

use crate::config::Config;
use crate::download::{check_version, latest_version};
use crate::errors::Result;
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
) -> Result<HttpResponse> {
    let (name, mut version) = path.into_inner();
    let Ok((extension_id, project_name)) = get_extension_id_and_name(&name, &conn).await else {
        return Ok(HttpResponse::NotFound().body("No extension with the given name was found"));
    };

    // Use latest version if 'latest' provided as version
    if version == "latest" {
        version = latest_version(extension_id as _, conn.as_ref()).await?;
    } else {
        let version_exists = check_version(conn.as_ref(), extension_id as _, &version).await?;

        if !version_exists {
            return Ok(HttpResponse::NotFound().body("Version not found"));
        }
    }
    let url = extension_location(&cfg.bucket_name, &project_name, &version);
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

/// Given an extension name, try to find it in the `extensions` table (more common scenario).
///
/// If it's not found, try to find it in `versions` under `extension_name`.
pub async fn get_extension_id_and_name<'n>(
    extension_name: &'n str,
    conn: &Pool<Postgres>,
) -> Result<(i64, Cow<'n, str>)> {
    if let Ok(record) = sqlx::query!("SELECT id FROM extensions WHERE name = $1", extension_name)
        .fetch_one(conn)
        .await
    {
        return Ok((record.id, extension_name.into()));
    }

    let record = sqlx::query!(
        "SELECT extension_id FROM versions WHERE extension_name = $1",
        extension_name
    )
    .fetch_one(conn)
    .await?;

    // Safe unwrap: if `extension_name` is in versions, `extension_id` must be as well
    let extension_id = record.extension_id.unwrap() as i64;

    let record = sqlx::query!("SELECT name FROM extensions WHERE id = $1", extension_id)
        .fetch_one(conn)
        .await?;

    // Safe unwrap: if the `extension` has an entry in `versions` it must have its
    // name filled in
    Ok((extension_id, record.name.unwrap().into()))
}
