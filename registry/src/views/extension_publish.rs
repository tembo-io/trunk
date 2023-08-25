//! This module handles the expected information an extension should have
use std::collections::HashMap;

use serde::Deserialize;

pub type SystemDependencies = HashMap<String, Vec<String>>;

#[derive(Deserialize, Debug)]
pub struct ExtensionUpload {
    pub name: String,
    pub extension_name: Option<String>,
    pub vers: semver::Version,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub categories: Option<Vec<String>>,
    pub system_dependencies: Option<SystemDependencies>,
}
