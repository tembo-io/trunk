use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::Deserialize;

use crate::{
    errors::Result,
    repository::Registry,
    v1::repository::TrunkProjectView,
};

// For some reason this is flagged as unused but it is actually used by
// OpenAPI spec generation.
#[allow(unused)]
use crate::v1::repository::TrunkProjectViews;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ExtensionNameQueryParams {
    extension_name: Option<String>,
}

#[utoipa::path(
    responses(
        (status = 200, body = TrunkProjectViews, content_type = "application/json", description = "A list of the latest versions of Trunk projects"),
    ),
    params(("extension-name" = Option<String>, Query, description = "filter Trunk projects by extension name"),),
)]
/// Retrieve a list of the latest versions of trunk projects.
///
/// Optional query parameter: extension_name:
/// * needed when trying to find dependencies of an extension but we only know its extension name, not its Trunk project name
#[get("/trunk-projects")]
pub async fn all_trunk_projects(
    registry: web::Data<Registry>,
    query: web::Query<ExtensionNameQueryParams>,
) -> Result<Json<Vec<TrunkProjectView>>> {
    if let Some(extension_name) = &query.extension_name {
        let trunk_projects = registry
            .trunk_projects_by_extension_name(extension_name)
            .await?;
        Ok(Json(trunk_projects))
    } else {
        let trunk_projects = registry.all_trunk_projects().await?;
        Ok(Json(trunk_projects))
    }
}

#[utoipa::path(
    responses(
        (status = 200, body = TrunkProjectViews, content_type = "application/json", description = "A list of the latest versions of Trunk projects"),
    ),
    params(
        ("trunk_project_name", description = "Trunk project name"),
    )
)]
/// Retrieve a list of all versions of the Trunk project with the given name.
#[get("/trunk-projects/{trunk_project_name}")]
pub async fn trunk_projects_by_name(
    registry: web::Data<Registry>,
    path: web::Path<String>,
) -> Result<Json<Vec<TrunkProjectView>>> {
    let trunk_project_name = path.into_inner();
    let trunk_projects = registry.trunk_projects_by_name(&trunk_project_name).await?;

    Ok(Json(trunk_projects))
}

#[utoipa::path(
    responses(
        (status = 200, body = TrunkProjectViews, content_type = "application/json", description = "A list of the latest versions of Trunk projects"),
    ),
    params(
        ("trunk_project_name", description = "Trunk project name"),
        ("version", description = "Trunk project version"),
    )
)]
/// Retrieve info on the Trunk project with the given name and version
#[get("/trunk-projects/{trunk_project_name}/version/{version}")]
pub async fn trunk_project_by_name_and_version(
    registry: web::Data<Registry>,
    path: web::Path<(String, String)>,
) -> Result<Json<Vec<TrunkProjectView>>> {
    let (trunk_project_name, trunk_project_version) = path.into_inner();
    let trunk_projects = registry
        .trunk_projects_by_name_and_version(&trunk_project_name, &trunk_project_version)
        .await?;

    Ok(Json(trunk_projects))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Information successfully inserted"),
    ),
    request_body = TrunkProjectView,
)]
/// Post a new Trunk project version
#[post("/trunk-projects")]
pub async fn insert_trunk_project(
    registry: web::Data<Registry>,
    body: web::Json<TrunkProjectView>,
    _auth: BearerAuth,
) -> Result<HttpResponse> {
    let trunk_project_to_insert = body.into_inner();

    registry
        .insert_trunk_project(trunk_project_to_insert)
        .await?;

    Ok(HttpResponse::Ok().finish())
}
