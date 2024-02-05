use log::warn;
use serde::{Deserialize, Serialize};
use std::io::Read;

use crate::trunk_toml::TrunkToml;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionConfiguration {
    #[serde(alias = "name")]
    configuration_name: String,
    #[serde(default)]
    #[serde(alias = "required")]
    is_required: bool,
    #[serde(alias = "default")]
    recommended_default_value: Option<String>,
}

pub fn parse_trunk_toml<R: Read>(mut reader: R) -> Result<TrunkToml, anyhow::Error> {
    let mut body = String::new();
    reader.read_to_string(&mut body)?;

    match toml::from_str(&body) {
        Ok(toml) => Ok(toml),
        Err(e) => {
            warn!("Trunk.toml is not valid toml");
            Err(e.into())
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoadableLibrary {
    pub library_name: String,
    pub requires_restart: bool,
    // set default to 2147483647
    #[serde(default = "default_priority")]
    pub priority: Option<i32>,
}

fn default_priority() -> Option<i32> {
    Some(2147483647)
}

#[cfg(test)]
mod tests {
    use crate::config::parse_trunk_toml;

    #[test]
    fn test_parse_trunk_toml_valid() {
        let toml = r#"
        [extension]
        name = "pg_cron"
        version = "1.5.2"
        repository = "https://github.com/citusdata/pg_cron"
        license = "PostgreSQL"
        description = "Run periodic jobs in PostgreSQL"
        homepage = "https://github.com/citusdata/pg_cron"
        documentation = "https://github.com/citusdata/pg_cron"
        categories = ["analytics", "debugging"]
        registry = "https://my.dummy.registry.dev"


        [build]
        postgres_version = "15"
        platform = "linux/amd64"
        dockerfile = "Dockerfile"
        install_command = "cd pg_cron && make install"
        "#;
        let table = parse_trunk_toml(toml.as_bytes()).unwrap();
        assert_eq!(table.extension.name, "pg_cron");
        assert_eq!(table.extension.version, "1.5.2");
        assert_eq!(
            table.extension.repository.unwrap(),
            "https://github.com/citusdata/pg_cron"
        );

        assert_eq!(table.extension.license, "PostgreSQL");
        assert_eq!(
            table.extension.description.unwrap(),
            "Run periodic jobs in PostgreSQL"
        );
        assert_eq!(
            table.extension.homepage.unwrap(),
            "https://github.com/citusdata/pg_cron"
        );
        assert_eq!(
            table.extension.documentation.unwrap(),
            "https://github.com/citusdata/pg_cron"
        );
        let categories = table.extension.categories;
        assert_eq!(categories, ["analytics", "debugging"]);
        assert_eq!(
            table.extension.registry.unwrap(),
            "https://my.dummy.registry.dev"
        );
        assert_eq!(table.build.dockerfile.unwrap(), "Dockerfile");
        assert_eq!(
            table.build.install_command.unwrap(),
            "cd pg_cron && make install"
        );
    }

    #[test]
    fn test_parse_trunk_toml_invalid() {
        let toml = "this is not valid toml";
        let result = parse_trunk_toml(toml.as_bytes());
        assert!(result.is_err());
    }
}
