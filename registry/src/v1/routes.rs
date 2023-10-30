use actix_web::{
    get,
    web::{self, Json},
};
use serde::Deserialize;

use crate::{errors::Result, repository::Registry, v1::repository::TrunkProjectView};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ExtensionNameQueryParams {
    extension_name: Option<String>,
}

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
