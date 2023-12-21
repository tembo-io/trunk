use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::config::{ExtensionConfiguration, LoadableLibrary};

/// Packaged file
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum PackagedFile {
    ControlFile {},
    SqlFile {},
    SharedObject {},
    Bitcode {},
    Extra {},
    LicenseFile {},
}

impl PackagedFile {
    pub fn from<P: AsRef<Path>>(path: P) -> Self {
        let extension = path.as_ref().extension();
        if path.as_ref().starts_with("licenses") {
            return PackagedFile::LicenseFile {};
        }
        if let Some(ext) = extension {
            match ext.to_str() {
                Some("control") => PackagedFile::ControlFile {},
                Some("sql") => PackagedFile::SqlFile {},
                Some("so") => PackagedFile::SharedObject {},
                Some("bc") => PackagedFile::Bitcode {},
                Some(_) | None => PackagedFile::Extra {},
            }
        } else {
            PackagedFile::Extra {}
        }
    }
}

/// Package manifest
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(Default))]
pub struct Manifest {
    pub name: String,
    pub extension_name: Option<String>,
    /// The packages that supply the runtime dependencies for this extension, if any are needed.
    /// Example: pgroonga depends on `libgroonga.so`, which on Ubuntu is supplied by `libgroonga-dev`
    /// This would be defined in manifest.json as
    ///
    /// ```no-rust
    /// dependencies = {
    ///    'apt': ['libgroonga-dev'],
    /// }
    /// ```
    pub extension_dependencies: Option<Vec<String>>,
    pub dependencies: Option<HashMap<String, Vec<String>>>,
    #[serde(rename = "version")]
    pub extension_version: String,
    pub manifest_version: i32,
    pub sys: String,
    pub architecture: String,
    pub files: Option<HashMap<PathBuf, PackagedFile>>,
    pub configurations: Option<Vec<ExtensionConfiguration>>,
    pub loadable_libraries: Option<Vec<LoadableLibrary>>,
    #[serde(default = "default_pg_version")]
    pub pg_version: u8,
}

const fn default_pg_version() -> u8 {
    15
}

impl Manifest {
    pub fn merge(&mut self, other: Self) {
        match &mut self.files {
            Some(current_files) => {
                if let Some(files_to_insert) = other.files {
                    current_files.extend(files_to_insert);
                }
            }
            None => self.files = other.files,
        }
    }

    pub fn add_file<P: AsRef<Path>>(&mut self, path: P) -> &mut PackagedFile {
        let files = match self.files {
            None => {
                self.files.replace(HashMap::new());
                self.files.as_mut().unwrap()
            }
            Some(ref mut files) => files,
        };
        files.insert(path.as_ref().to_owned(), PackagedFile::from(path.as_ref()));

        files.get_mut(path.as_ref()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::manifest::PackagedFile;

    use super::Manifest;

    #[test]
    fn adds_files_to_manifest() {
        let mut manifest = Manifest::default();
        assert!(manifest.files.is_none());

        manifest.add_file("/etc/some/file.control");
        manifest.add_file("./another/file");
        manifest.add_file("shared_rustc.so");
        manifest.add_file("program.bc");
        manifest.add_file("delete_prod_bank.sql");
        manifest.add_file("licenses");

        for (path, file_kind) in manifest.files.unwrap() {
            match file_kind {
                PackagedFile::ControlFile {} => {
                    assert_eq!(path, Path::new("/etc/some/file.control"))
                }
                PackagedFile::SqlFile {} => assert_eq!(path, Path::new("delete_prod_bank.sql")),
                PackagedFile::SharedObject {} => assert_eq!(path, Path::new("shared_rustc.so")),
                PackagedFile::Bitcode {} => assert_eq!(path, Path::new("program.bc")),
                PackagedFile::Extra {} => assert_eq!(path, Path::new("./another/file")),
                PackagedFile::LicenseFile {} => assert_eq!(path, Path::new("licenses")),
            }
        }
    }

    #[test]
    fn merges_manifests() {
        let mut manifest_1 = Manifest::default();
        let mut manifest_2 = Manifest::default();

        manifest_1.add_file("manifest.json");
        manifest_2.add_file("pgmq.control");

        // Merge manifest_2 into manifest_1
        manifest_1.merge(manifest_2);

        let files = manifest_1.files.unwrap();

        assert!(files.contains_key(Path::new("manifest.json")));
        assert!(files.contains_key(Path::new("pgmq.control")));
    }
}
