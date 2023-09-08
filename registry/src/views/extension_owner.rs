use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionOwner {
    pub user_id: String,
    pub user_name: String,
}
