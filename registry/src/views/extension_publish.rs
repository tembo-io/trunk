//! This module handles the expected information an extension should have
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
    pub libraries: Option<Vec<LoadableLibrary>>,
    pub configurations: Option<Vec<ExtensionConfiguration>>,
    pub control_file: Option<ControlFileMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoadableLibrary {
    pub library_name: String,
    pub requires_restart: bool,
    pub priority: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExtensionConfiguration {
    #[serde(alias = "name")]
    pub configuration_name: String,
    pub is_required: bool,
    #[serde(alias = "default")]
    pub recommended_default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ControlFileMetadata {
    pub absent: bool,
    pub content: Option<String>,
}
