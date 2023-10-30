use actix_web::{
    get,
    web::{self, Json},
};
use serde::Deserialize;

use crate::{errors::Result, repository::Registry, v1::repository::TrunkProjectView};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Params {
    extension_name: Option<String>,
}

/// Retrieve a list of the latest versions of trunk projects.
///
/// Optional query parameter: extension_name:
/// * needed when trying to find dependencies of an extension but we only know its extension name, not its Trunk project name
#[get("/trunk-projects")]
pub async fn all_trunk_projects(
    registry: web::Data<Registry>,
    query: web::Query<Params>,
) -> Result<Json<Vec<TrunkProjectView>>> {
    tracing::info!("Got params: {:?}", query);
    tracing::info!("Query is {:?}", query.extension_name);
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
