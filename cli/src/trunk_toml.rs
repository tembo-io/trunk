use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use glob::PatternError;

use crate::config::{ExtensionConfiguration, LoadableLibrary};

pub type SystemDependencies = HashMap<String, Vec<String>>;

/// The definition/schema of a Trunk.toml file
#[derive(Serialize, Deserialize, Debug)]
pub struct TrunkToml {
    pub extension: TomlExtensionData,
    pub build: TomlBuildInfo,
    pub dependencies: Option<SystemDependencies>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlExtensionData {
    pub name: String,
    pub file: Option<PathBuf>,
    pub extension_name: Option<String>,
    pub extension_dependencies: Option<Vec<String>>,
    pub version: String,
    pub license: String,
    pub repository: Option<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub categories: Vec<String>,
    pub registry: Option<String>,
    pub preload_libraries: Option<Vec<String>>,
    pub configurations: Option<Vec<ExtensionConfiguration>>,
    pub loadable_libraries: Option<Vec<LoadableLibrary>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlBuildInfo {
    pub postgres_version: Option<String>,
    pub platform: String,
    /// List of globs to package extra files into the archive.
    /// This is useful if you need to package in a file that is not a .sql, .so, .bc, and so on.
    ///
    /// Example:
    ///
    /// ```
    /// include = ["*.data"]
    /// ```
    pub include: Option<Vec<String>>,
    pub dockerfile: Option<String>,
    pub install_command: Option<String>,
}

impl TomlBuildInfo {
    pub fn build_glob_patterns(&self) -> Result<Vec<glob::Pattern>, PatternError> {
        let Some(patterns) = &self.include else {
            return Ok(Vec::new());
        };

        patterns
            .iter()
            .map(|pattern| glob::Pattern::new(pattern))
            .collect()
    }
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
        .or_else(|| maybe_toml.as_ref().map(extract).and_then(Option::as_ref))
        .cloned()
}
