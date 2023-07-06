use std::io::Read;
use toml::Table;

pub fn get_from_trunk_toml_if_not_set_on_cli(
    cli_setting: Option<String>,
    trunk_toml: Option<Table>,
    table_name: &str,
    key: &str,
) -> Option<String> {
    match cli_setting {
        Some(cli_setting) => Some(cli_setting),
        None => match trunk_toml {
            Some(table) => match table.get(table_name) {
                Some(extension) => match extension.get(key) {
                    Some(value) => {
                        let result = value
                            .as_str()
                            .unwrap_or_else(|| {
                                panic!("Trunk.toml: {}.{} should be a string", table_name, key)
                            })
                            .to_string();
                        println!(
                            "Trunk.toml: using setting {}.{}: {}",
                            table_name, key, result
                        );
                        Some(result)
                    }
                    None => {
                        println!("Trunk.toml: {}.{} is not set", table_name, key);
                        None
                    }
                },
                None => None,
            },
            None => None,
        },
    }
}

pub fn get_string_vec_from_trunk_toml_if_not_set_on_cli(
    cli_setting: Option<Vec<String>>,
    trunk_toml: Option<Table>,
    table_name: &str,
    key: &str,
) -> Option<Vec<String>> {
    match cli_setting {
        Some(cli_setting) => Some(cli_setting),
        None => match trunk_toml.clone() {
            Some(table) => match table.get(table_name) {
                Some(extension) => match extension.get(key) {
                    Some(value) => {
                        let result = value.as_array().unwrap_or_else(|| {
                            panic!(
                                "Trunk.toml: {}.{} should be an array of strings",
                                table_name, key
                            )
                        });
                        let mut v: Vec<String> = Vec::new();
                        for i in result {
                            let s = i.as_str();
                            let s = s.unwrap().to_string();
                            v.push(s);
                        }
                        println!("Trunk.toml: using setting {}.{}: {:?}", table_name, key, v);
                        Some(v)
                    }
                    None => None,
                },
                None => None,
            },
            None => None,
        },
    }
}

pub fn parse_trunk_toml<R: Read>(mut reader: R) -> Result<Option<Table>, anyhow::Error> {
    let mut body = String::new();
    reader.read_to_string(&mut body)?;
    match toml::from_str::<Table>(&body) {
        Ok(table) => Ok(Some(table)),
        Err(e) => {
            println!("Trunk.toml is not valid toml");
            Err(e.into())
        }
    }
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
        dockerfile = "Dockerfile"
        install_command = "cd pg_cron && make install"
        "#;
        let result = parse_trunk_toml(toml.as_bytes()).unwrap();
        let table = result.expect("Expected a table");
        assert_eq!(table["extension"]["name"].as_str().unwrap(), "pg_cron");
        assert_eq!(table["extension"]["version"].as_str().unwrap(), "1.5.2");
        assert_eq!(
            table["extension"]["repository"].as_str().unwrap(),
            "https://github.com/citusdata/pg_cron"
        );
        assert_eq!(
            table["extension"]["license"].as_str().unwrap(),
            "PostgreSQL"
        );
        assert_eq!(
            table["extension"]["description"].as_str().unwrap(),
            "Run periodic jobs in PostgreSQL"
        );
        assert_eq!(
            table["extension"]["homepage"].as_str().unwrap(),
            "https://github.com/citusdata/pg_cron"
        );
        assert_eq!(
            table["extension"]["documentation"].as_str().unwrap(),
            "https://github.com/citusdata/pg_cron"
        );
        let categories = table["extension"]["categories"].as_array().unwrap();
        let mut v: Vec<String> = Vec::new();
        for i in categories {
            let s = i.as_str();
            let s = s.unwrap().to_string();
            v.push(s);
        }
        assert_eq!(v, vec!("analytics", "debugging"));
        assert_eq!(
            table["extension"]["registry"].as_str().unwrap(),
            "https://my.dummy.registry.dev"
        );

        assert_eq!(table["build"]["dockerfile"].as_str().unwrap(), "Dockerfile");
        assert_eq!(
            table["build"]["install_command"].as_str().unwrap(),
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
