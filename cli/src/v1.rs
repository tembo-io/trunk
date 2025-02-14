use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TrunkProjectView {
    pub name: String,
    pub version: String,
    #[expect(unused)]
    pub postgres_versions: Option<Vec<u8>>,
    pub downloads: Option<Vec<Download>>,
    pub extensions: Vec<Extension>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Extension {
    pub extension_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Download {
    pub link: String,
    pub pg_version: u8,
    #[expect(unused)]
    pub platform: String,
    pub sha256: String,
}
