use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserInfo {
    pub user_id: String,
    pub user_name: String,
}
