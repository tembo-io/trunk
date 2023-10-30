use actix_web::{
    get,
    web::{self, Json}
};
use serde::Deserialize;

use crate::{errors::Result, repository::Registry, v1::repository::TrunkProjectView};

#[derive(Debug, Deserialize)]
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
    if let Some(_extension_name) = &query.extension_name {
        // SELECT
        Ok(Json(Vec::new()))
    } else {
        let trunk_projects = registry.all_trunk_projects().await?;
        Ok(Json(trunk_projects))
    }
}
