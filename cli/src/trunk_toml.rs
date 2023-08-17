use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

/// The definition/schema of a Trunk.toml file
#[derive(Serialize, Deserialize, Debug)]
pub struct TrunkToml {
    pub extension: TomlExtensionData,
    pub build: TomlBuildInfo,
    pub dependencies: Option<HashMap<String, Vec<String>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlExtensionData {
    pub name: String,
    pub file: Option<PathBuf>,
    pub extension_name: Option<String>,
    pub version: String,
    pub license: String,
    pub repository: Option<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub categories: Vec<String>,
    pub registry: Option<String>,
    pub shared_preload_libraries: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlBuildInfo {
    pub postgres_version: Option<String>,
    pub platform: String,
    pub dockerfile: Option<String>,
    pub install_command: String,
}

/// Use the value supplied through the command-line, if present.
/// If not, fallback to the value specified in the Trunk.toml
pub(crate) fn cli_or_trunk<T: Clone, F: FnOnce(&TrunkToml) -> &T>(
    set_in_cli: &Option<T>,
    extract: F,
    maybe_toml: &Option<TrunkToml>,
) -> Option<T> {
    set_in_cli
        .as_ref()
        .or_else(|| maybe_toml.as_ref().map(extract))
        .cloned()
}

/// Use the value supplied through the command-line, if present.
/// If not, fallback to the value specified in the Trunk.toml, if present
pub(crate) fn cli_or_trunk_opt<T: Clone, F: FnOnce(&TrunkToml) -> &Option<T>>(
    set_in_cli: &Option<T>,
    extract: F,
    maybe_toml: &Option<TrunkToml>,
) -> Option<T> {
    set_in_cli
        .as_ref()
        .or_else(|| {
            maybe_toml
                .as_ref()
                .map(extract)
                .map(Option::as_ref)
                .flatten()
        })
        .cloned()
}
