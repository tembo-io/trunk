use utoipa::openapi::License;
use utoipa::OpenApi;

use crate::v1::repository::{ExtensionView, TrunkProjectView};
use crate::v1::routes as v1;
use crate::views::extension_publish::{ExtensionConfiguration, LoadableLibrary};

// v1 API documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        v1::all_trunk_projects,
        v1::trunk_projects_by_name,
        v1::trunk_project_by_name_and_version,
        v1::insert_trunk_project,
    ),
    components(schemas(
        ExtensionView,
        LoadableLibrary,
        ExtensionConfiguration,
        TrunkProjectView,
    ))
)]
pub struct V1ApiDoc;

pub fn build_docs() -> utoipa::openapi::OpenApi {
    let mut v1_docs = V1ApiDoc::openapi();
    v1_docs.info.title = "Trunk Registry".to_string();
    v1_docs.info.license = Some(License::new("MIT"));
    v1_docs.info.description = Some(
        r#"Trunk Registry API 
        </br>
        <p align="center">
          <img src="https://github.com/tembo-io/trunk/assets/8935584/905ef1f3-10ff-48b5-90af-74af74ebb1b1" width=25% height=25%>
        </p>
        </br>
        "#
        .to_string(),
    );

    v1_docs
}
