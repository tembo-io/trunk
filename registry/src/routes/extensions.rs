//! Functionality related to publishing a new extension or version of an extension.

use crate::config::Config;
use crate::download::latest_version;
use crate::errors::ExtensionRegistryError;
use crate::extensions::{add_extension_owner, check_input, extension_owners, latest_license};
use crate::token::validate_token;
use crate::uploader::upload_extension;
use crate::views::extension_publish::ExtensionUpload;
use crate::views::user_info::UserInfo;
use actix_multipart::Multipart;
use actix_web::http::header::AUTHORIZATION;
use actix_web::{error, get, post, web, HttpResponse};
use aws_config::SdkConfig;
use aws_sdk_s3;
use aws_sdk_s3::primitives::ByteStream;
use futures::TryStreamExt;
use log::{error, info};
use serde_json::{json, Value};
use sqlx::{Pool, Postgres};

const MAX_SIZE: usize = 5000000; // max payload size is 5M

/// Handles the `POST /extensions/new` route.
/// Used by `trunk publish` to publish a new extension or to publish a new version of an
/// existing extension.

#[post("/extensions/new")]
pub async fn publish(
    cfg: web::Data<Config>,
    conn: web::Data<Pool<Postgres>>,
    aws_config: web::Data<SdkConfig>,
    mut payload: Multipart,
) -> Result<HttpResponse, ExtensionRegistryError> {
    let mut metadata = web::BytesMut::new();
    let mut file = web::BytesMut::new();
    let mut user_info = UserInfo {
        user_id: "".to_string(),
        user_name: "".to_string(),
    };

    // Get request body
    while let Some(mut field) = payload.try_next().await? {
        let headers = field.headers();
        let auth = headers.get(AUTHORIZATION).unwrap();
        // Check if token exists and has an associated user
        user_info = validate_token(auth, conn.clone()).await?;
        // Field is stream of Bytes
        while let Some(chunk) = field.try_next().await? {
            // limit max size of in-memory payload
            if (chunk.len()) > MAX_SIZE {
                return Err(ExtensionRegistryError::from(error::ErrorBadRequest(
                    "extension size is greater than 5M",
                )));
            }
            if field.name() == "metadata" {
                metadata.extend_from_slice(&chunk);
            } else if field.name() == "file" {
                file.extend_from_slice(&chunk);
            }
        }
    }

    // Deserialize body
    let new_extension = serde_json::from_slice::<ExtensionUpload>(&metadata)?;

    // Create a transaction on the database, if there are no errors,
    // commit the transactions to record a new or updated extension.
    let mut tx = conn.begin().await?;

    // Validate name input
    check_input(&new_extension.name)?;

    // Check if extension exists
    let exists = sqlx::query!(
        "SELECT * FROM extensions WHERE name = $1",
        new_extension.name
    )
    .fetch_optional(&mut tx)
    .await?;

    match exists {
        // TODO(ianstanton) Refactor into separate functions
        Some(exists) => {
            // Extension exists
            let mut tx = conn.begin().await?;
            let extension_id = exists.id;
            // Check if extension has an owner
            let has_owner = sqlx::query!(
                "SELECT *
                FROM extension_owners
                WHERE
                    extension_id = $1",
                extension_id as i32
            )
            .fetch_optional(&mut tx)
            .await?;
            match has_owner {
                Some(_has_owner) => Ok::<(), ExtensionRegistryError>(()),
                None => {
                    // The extension has no owner. Add user ID as owner of this extension.
                    info!(
                        "The extension {} exists and has no owner. Adding {} as an owner of this extension.",
                        new_extension.name, user_info.user_id
                    );
                    add_extension_owner(
                        extension_id,
                        &user_info.user_id,
                        &user_info.user_name,
                        conn,
                    )
                    .await?;
                    Ok(())
                }
            }?;
            // Check if user is owner of extension
            let is_owner = sqlx::query!(
                "SELECT *
                FROM extension_owners
                WHERE
                    extension_id = $1
                    and owner_id = $2",
                extension_id as i32,
                user_info.user_id
            )
            .fetch_optional(&mut tx)
            .await?;
            match is_owner {
                Some(_is_owner) => Ok(()),
                None => {
                    error!(
                        "The user associated with this API token is not an owner of extension {}",
                        new_extension.name
                    );
                    Err(ExtensionRegistryError::AuthorizationError(format!(
                        "The user associated with this API token is not an owner of extension {}",
                        new_extension.name,
                    )))
                }
            }?;
            // Check if version exists
            let version_exists = sqlx::query!(
                "SELECT *
                FROM versions
                WHERE 
                    extension_id = $1
                    and num = $2",
                extension_id as i32,
                new_extension.vers.to_string()
            )
            .fetch_optional(&mut tx)
            .await?;

            match version_exists {
                Some(_version_exists) => {
                    // Update updated_at timestamp
                    sqlx::query!(
                        "UPDATE versions
                    SET updated_at = (now() at time zone 'utc'), license = $1, published_by = $2
                    WHERE extension_id = $3
                    AND num = $4",
                        new_extension.license,
                        user_info.user_name,
                        extension_id as i32,
                        new_extension.vers.to_string()
                    )
                    .execute(&mut tx)
                    .await?;
                }
                None => {
                    // Create new record in versions table
                    sqlx::query!(
                        "
                    INSERT INTO versions(extension_id, num, created_at, yanked, license, published_by)
                    VALUES ($1, $2, (now() at time zone 'utc'), $3, $4, $5)
                    ",
                        extension_id as i32,
                        new_extension.vers.to_string(),
                        false,
                        new_extension.license,
                        user_info.user_name
                    )
                    .execute(&mut tx)
                    .await?;
                }
            }

            // Set updated_at time on extension
            sqlx::query!(
                "UPDATE extensions
            SET updated_at = (now() at time zone 'utc'), description = $1, documentation = $2, homepage = $3, repository = $4
            WHERE name = $5",
                new_extension.description,
                new_extension.documentation,
                new_extension.homepage,
                new_extension.repository,
                new_extension.name,
            )
            .execute(&mut tx)
            .await?;
            tx.commit().await?;
        }
        None => {
            // Else, create new record in extensions table
            let mut tx = conn.begin().await?;
            let id_row = sqlx::query!(
                "
            INSERT INTO extensions(name, created_at, updated_at, description, homepage, documentation, repository)
            VALUES ($1, (now() at time zone 'utc'), (now() at time zone 'utc'), $2, $3, $4, $5)
            RETURNING id
            ",
                new_extension.name,
                new_extension.description,
                new_extension.homepage,
                new_extension.documentation,
                new_extension.repository
            )
            .fetch_one(&mut tx)
            .await?;
            let extension_id = id_row.id;

            // Create new record in versions table
            sqlx::query!(
                "
            INSERT INTO versions(extension_id, num, created_at, yanked, license, published_by)
            VALUES ($1, $2, (now() at time zone 'utc'), $3, $4, $5)
            ",
                extension_id as i32,
                new_extension.vers.to_string(),
                false,
                new_extension.license,
                user_info.user_name
            )
            .execute(&mut tx)
            .await?;
            tx.commit().await?;
            // Set user ID as an owner of the new extension
            info!(
                "Adding {} as an owner of new extension {}.",
                user_info.user_id, new_extension.name
            );
            add_extension_owner(extension_id, &user_info.user_id, &user_info.user_name, conn)
                .await?;
        }
    }

    // TODO(ianstanton) Generate checksum
    let file_byte_stream = ByteStream::from(file.freeze());
    let client = aws_sdk_s3::Client::new(&aws_config);
    upload_extension(
        &cfg.bucket_name,
        &client,
        file_byte_stream,
        &new_extension,
        &new_extension.vers,
    )
    .await?;
    Ok(HttpResponse::Ok().body(format!(
        "Successfully published extension {} version {}",
        new_extension.name, new_extension.vers
    )))
}

#[get("/extensions/all")]
pub async fn get_all_extensions(
    conn: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, ExtensionRegistryError> {
    let mut extensions: Vec<Value> = Vec::new();

    // Create a database transaction
    let mut tx = conn.begin().await?;
    let rows = sqlx::query!("SELECT * FROM extensions")
        .fetch_all(&mut tx)
        .await?;
    for row in rows.iter() {
        let name = row.name.to_owned().unwrap();
        let version = latest_version(&name, conn.clone()).await?;
        let license = latest_license(&name, conn.clone()).await?;
        let owners = extension_owners(&name, conn.clone()).await?;
        let data = json!(
        {
          "name": row.name.to_owned(),
          "latestVersion": version,
          "createdAt": row.created_at.to_string(),
          "updatedAt": row.updated_at.to_string(),
          "description": row.description.to_owned(),
          "homepage": row.homepage.to_owned(),
          "documentation": row.documentation.to_owned(),
          "repository": row.repository.to_owned(),
          "license": license,
          "owners": owners
        });
        extensions.push(data);
    }
    // Return results in response
    let json = serde_json::to_string_pretty(&extensions)?;
    Ok(HttpResponse::Ok().body(json))
}

#[get("/extensions/detail/{extension_name}")]
pub async fn get_version_history(
    conn: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse, ExtensionRegistryError> {
    let name = path.into_inner();
    let mut versions: Vec<Value> = Vec::new();

    // Create a database transaction
    let mut tx = conn.begin().await?;
    // Get extension information
    let row = sqlx::query!("SELECT * FROM extensions WHERE name = $1", name)
        .fetch_one(&mut tx)
        .await?;
    let id: i32 = row.id as i32;
    let name = row.name.to_owned().unwrap();
    let description = row.description.to_owned();
    let homepage = row.homepage.to_owned();
    let documentation = row.documentation.to_owned();
    let repository = row.repository.to_owned();
    let owners = extension_owners(&name, conn.clone()).await?;

    // Get information for all versions of extension
    let rows = sqlx::query!("SELECT * FROM versions WHERE extension_id = $1", id)
        .fetch_all(&mut tx)
        .await?;
    for row in rows.iter() {
        let data = json!(
        {
          "name": name.to_owned(),
          "version": row.num,
          "createdAt": row.created_at.to_string(),
          "updatedAt": row.updated_at.to_string(),
          "description": description,
          "homepage": homepage,
          "documentation": documentation,
          "repository": repository,
          "license": row.license,
          "owners": owners,
          "publisher": row.published_by
        });
        versions.push(data);
    }
    // Return results in response
    let json = serde_json::to_string_pretty(&versions)?;
    Ok(HttpResponse::Ok().body(json))
}
