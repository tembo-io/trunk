use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TrunkProjectView {
    pub name: String,
    pub version: String,
    pub postgres_versions: Option<Vec<u8>>,
    pub downloads: Option<Vec<Download>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Download {
    pub link: String,
    pub pg_version: u8,
    pub platform: String,
    pub sha256: String,
}
