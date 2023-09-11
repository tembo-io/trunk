//{
//    "name": row.name,
//    "latestVersion": version,
//    "createdAt": row.created_at.to_string(),
//    "updatedAt": row.updated_at.to_string(),
//    "description": row.description,
//    "homepage": row.homepage,
//    "documentation": row.documentation,
//    "repository": row.repository,
//    "license": license,
//    "owners": owners,
//    "categories": categories
//  });

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionOwner {
    pub user_id: String,
    pub user_name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionDetails {
    pub name: Option<String>,
    #[serde(rename = "camelCase")]
    pub latest_version: String,
    #[serde(rename = "camelCase")]
    pub created_at: String,
    #[serde(rename = "camelCase")]
    pub updated_at: String,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub license: String,
    pub owners: Vec<ExtensionOwner>,
    pub categories: Vec<String>,
}
