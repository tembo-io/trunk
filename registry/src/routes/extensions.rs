//! Functionality related to publishing a new extension or version of an extension.

use actix_web::web::Json;
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::categories::{get_categories_for_extension, update_extension_categories};
use crate::config::Config;
use crate::errors::ExtensionRegistryError;
use crate::extensions::{add_extension_owner, check_input, extension_owners};
use crate::readme::GithubApiClient;
use crate::repository::Registry;
use crate::token::validate_token;
use crate::uploader::upload_extension;
use crate::v1::repository::TrunkProjectView;
use crate::views::extension_publish::ExtensionUpload;
use crate::views::user_info::UserInfo;
use actix_multipart::Multipart;
use actix_web::http::header::AUTHORIZATION;
use actix_web::{delete, error, get, post, web, HttpResponse};
use aws_config::SdkConfig;
use aws_sdk_s3;
use aws_sdk_s3::primitives::ByteStream;
use futures::TryStreamExt;
use serde::ser::Serializer;
use serde_json::{json, Value};
use sqlx::types::chrono::Utc;
use sqlx::{Pool, Postgres};
use tracing::{error, info};

const MAX_SIZE: usize = 15000000; // max payload size is 15M

/// Handles the `POST /extensions/new` route.
/// Used by `trunk publish` to publish a new extension or to publish a new version of an
/// existing extension.

#[post("/extensions/new")]
pub async fn publish(
    cfg: web::Data<Config>,
    conn: web::Data<Pool<Postgres>>,
    registry: web::Data<Registry>,
    github_client: web::Data<GithubApiClient>,
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

    let system_deps = serde_json::to_value(&new_extension.system_dependencies)?;
    let libraries = serde_json::to_value(&new_extension.libraries)?;

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
                        conn.clone(),
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
                    SET updated_at = (now() at time zone 'utc'), license = $1, published_by = $2, extension_name = $5, system_dependencies = $6::jsonb, libraries = $7::jsonb
                    WHERE extension_id = $3
                    AND num = $4",
                        new_extension.license,
                        user_info.user_name,
                        extension_id as i32,
                        new_extension.vers.to_string(),
                        new_extension.extension_name,
                        system_deps,
                        libraries,
                    )
                    .execute(&mut tx)
                    .await?;
                }
                None => {
                    // Create new record in versions table
                    sqlx::query!(
                        "
                    INSERT INTO versions(extension_id, num, created_at, yanked, license, published_by, extension_name, system_dependencies, libraries)
                    VALUES ($1, $2, (now() at time zone 'utc'), $3, $4, $5, $6, $7::jsonb, $8::jsonb)
                    ",
                        extension_id as i32,
                        new_extension.vers.to_string(),
                        false,
                        new_extension.license,
                        user_info.user_name,
                        new_extension.extension_name,
                        system_deps,
                        libraries
                    )
                    .execute(&mut tx)
                    .await?;
                }
            }

            // If categories is not None, update
            if new_extension.categories.is_some() {
                update_extension_categories(
                    extension_id,
                    new_extension.categories.clone().unwrap(),
                    conn.clone(),
                )
                .await?
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
            INSERT INTO versions(extension_id, num, created_at, yanked, license, published_by, extension_name, system_dependencies, libraries)
            VALUES ($1, $2, (now() at time zone 'utc'), $3, $4, $5, $6, $7::jsonb, $8::jsonb)
            ",
                extension_id as i32,
                new_extension.vers.to_string(),
                false,
                new_extension.license,
                user_info.user_name,
                new_extension.extension_name,
                system_deps,
                libraries
            )
            .execute(&mut tx)
            .await?;
            tx.commit().await?;

            // If categories not None, update
            if new_extension.categories.is_some() {
                update_extension_categories(
                    extension_id,
                    new_extension.categories.clone().unwrap(),
                    conn.clone(),
                )
                .await?
            }

            // Set user ID as an owner of the new extension
            info!(
                "Adding {} as an owner of new extension {}.",
                user_info.user_id, new_extension.name
            );
            add_extension_owner(extension_id, &user_info.user_id, &user_info.user_name, conn)
                .await?;
        }
    }

    // The uploaded contents in .tar.gz
    let gzipped_archive = file.freeze();

    // TODO(ianstanton) Generate checksum
    let file_byte_stream = ByteStream::from(gzipped_archive.clone());
    let client = aws_sdk_s3::Client::new(&aws_config);
    upload_extension(
        &cfg.bucket_name,
        &client,
        file_byte_stream,
        &new_extension,
        &new_extension.vers,
    )
    .await?;

    let response = HttpResponse::Ok().body(format!(
        "Successfully published extension {} version {}",
        new_extension.name, new_extension.vers
    ));

    let _ = crate::readme::fetch_and_save_readme(
        github_client.as_ref(),
        registry.as_ref(),
        &new_extension.name,
    )
    .await
    .map_err(|err| error!("Failed to fetch README in /publish: {err}"));

    let _ = insert_into_v1(new_extension, registry.as_ref(), gzipped_archive.as_ref())
        .await
        .map_err(|err| error!("Failed to insert extension into v1 in /publish: {err}"));

    Ok(response)
}

async fn insert_into_v1(
    new_extension: ExtensionUpload,
    registry: &Registry,
    gzipped_archive: &[u8],
) -> anyhow::Result<()> {
    let extension_views =
        crate::v1::extractor::extract_extension_view(gzipped_archive, &new_extension)?;

    let trunk_project = TrunkProjectView {
        name: new_extension.name,
        description: new_extension.description.unwrap_or_default(),
        documentation_link: new_extension.documentation,
        repository_link: new_extension.repository.unwrap_or_default(),
        version: new_extension.vers.to_string(),
        // TODO: state in Trunk.toml the supported versions
        postgres_versions: None,
        extensions: extension_views,
    };

    if let Err(err) = registry.insert_trunk_project(trunk_project).await {
        anyhow::bail!("Failed to insert into v1 schema: {err}")
    }

    Ok(())
}

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionOwner {
    pub user_id: String,
    pub user_name: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct ExtensionDetails {
    pub categories: Option<Value>,
    #[serde(serialize_with = "serialize_with_offset")]
    pub createdAt: Option<chrono::DateTime<Utc>>,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub homepage: Option<String>,
    pub latestVersion: Option<String>,
    pub license: Option<String>,
    pub name: Option<String>,
    pub owners: Option<Value>,
    pub repository: Option<String>,
    #[serde(serialize_with = "serialize_with_offset")]
    pub updatedAt: Option<chrono::DateTime<Utc>>,
}

fn serialize_with_offset<S>(
    date: &Option<chrono::DateTime<Utc>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(d) => {
            let s = d.to_rfc3339();
            serializer.serialize_str(&s)
        }
        None => serializer.serialize_none(),
    }
}

#[get("/extensions/all")]
pub async fn get_all_extensions(
    conn: web::Data<Pool<Postgres>>,
) -> Result<Json<Vec<ExtensionDetails>>, ExtensionRegistryError> {
    match sqlx::query_as!(ExtensionDetails, "SELECT * FROM extension_detail_vw")
        .fetch_all(conn.get_ref())
        .await
    {
        Ok(extensions) => Ok(Json(extensions)),
        Err(e) => {
            error!("Error fetching extensions: {}", e);
            Err(ExtensionRegistryError::from(e))
        }
    }
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
    let extension_id: i32 = row.id as i32;
    let description = row.description.to_owned();
    let homepage = row.homepage.to_owned();
    let documentation = row.documentation.to_owned();
    let repository = row.repository.to_owned();
    let owners = extension_owners(extension_id, conn.clone()).await?;
    let categories = get_categories_for_extension(extension_id, conn).await?;

    // Get information for all versions of extension
    let rows = sqlx::query!(
        "SELECT * FROM versions WHERE extension_id = $1",
        extension_id
    )
    .fetch_all(&mut tx)
    .await?;

    // TODO(ianstanton) DRY
    for row in rows.iter() {
        let data = json!(
        {
          "name": name.to_owned(),
          "extension_name": row.extension_name.to_owned(),
          "version": row.num,
          "createdAt": row.created_at.to_string(),
          "updatedAt": row.updated_at.to_string(),
          "description": description,
          "homepage": homepage,
          "documentation": documentation,
          "repository": repository,
          "license": row.license,
          "owners": owners,
          "publisher": row.published_by,
          "system_dependencies": row.system_dependencies,
          "categories": categories,
          "libraries": row.libraries
        });
        versions.push(data);
    }
    // Return results in response
    let json = serde_json::to_string_pretty(&versions)?;
    Ok(HttpResponse::Ok().body(json))
}

#[get("/extensions/detail/{extension_name}/{version}")]
pub async fn get_version(
    conn: web::Data<Pool<Postgres>>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, ExtensionRegistryError> {
    let (name, version) = path.into_inner();
    let mut versions: Vec<Value> = Vec::new();

    // Create a database transaction
    let mut tx = conn.begin().await?;
    // Get extension information
    let row = sqlx::query!("SELECT * FROM extensions WHERE name = $1", name)
        .fetch_one(&mut tx)
        .await?;
    let extension_id = row.id as i32;
    let description = row.description.to_owned();
    let homepage = row.homepage.to_owned();
    let documentation = row.documentation.to_owned();
    let repository = row.repository.to_owned();
    let owners = extension_owners(extension_id, conn.clone()).await?;
    let categories = get_categories_for_extension(extension_id, conn).await?;

    // Get information for all versions of extension
    let rows = sqlx::query!(
        "SELECT * FROM versions WHERE extension_id = $1 AND num = $2",
        extension_id,
        version
    )
    .fetch_all(&mut tx)
    .await?;

    // TODO(ianstanton) DRY
    for row in rows.iter() {
        let data = json!(
        {
          "name": name.to_owned(),
          "extension_name": row.extension_name.to_owned(),
          "version": row.num,
          "createdAt": row.created_at.to_string(),
          "updatedAt": row.updated_at.to_string(),
          "description": description,
          "homepage": homepage,
          "documentation": documentation,
          "repository": repository,
          "license": row.license,
          "owners": owners,
          "publisher": row.published_by,
          "system_dependencies": row.system_dependencies,
          "categories": categories,
          "libraries": row.libraries
        });
        versions.push(data);
    }
    // Return results in response
    let json = serde_json::to_string_pretty(&versions)?;
    Ok(HttpResponse::Ok().body(json))
}

#[tracing::instrument(skip(registry, _auth))]
#[delete("/extensions/{extension_name}")]
pub async fn delete_extension(
    registry: web::Data<Registry>,
    path: web::Path<String>,
    _auth: BearerAuth,
) -> Result<HttpResponse, ExtensionRegistryError> {
    let ext_name = path.into_inner();

    let extension_id = registry
        .extension_id(&ext_name)
        .await?
        .ok_or(ExtensionRegistryError::ResourceNotFound)?;

    // Remove all information related to this extension
    registry.purge_extension(extension_id).await?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/extensions/libraries/{library}")]
pub async fn put_shared_preload_libraries(
    conn: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    _auth: BearerAuth,
) -> Result<HttpResponse, ExtensionRegistryError> {
    let library = path.into_inner();
    sqlx::query!(
        "INSERT INTO shared_preload_libraries(name)
        VALUES ($1)
        ON CONFLICT (name) DO NOTHING;",
        &library
    )
    .execute(conn.as_ref())
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/extensions/libraries")]
pub async fn get_shared_preload_libraries(
    conn: web::Data<Pool<Postgres>>,
) -> Result<Json<Vec<String>>, ExtensionRegistryError> {
    // Query to get extension names from the appropriate table.
    let rows = sqlx::query!("SELECT name FROM shared_preload_libraries")
        .fetch_all(conn.as_ref())
        .await?;

    // Iterate through the rows and extract the extension names.
    let extension_names = rows.into_iter().map(|row| row.name).collect();

    // Return the results in the response.
    Ok(Json(extension_names))
}
