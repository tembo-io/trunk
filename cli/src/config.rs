use std::io::Read;

use crate::manifest::TrunkToml;

pub fn parse_trunk_toml<R: Read>(mut reader: R) -> Result<TrunkToml, anyhow::Error> {
    let mut body = String::new();
    reader.read_to_string(&mut body)?;

    match toml::from_str(&body) {
        Ok(toml) => Ok(toml),
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
        //   let table = parse_trunk_toml(toml.as_bytes()).unwrap();
        //   assert_eq!(table["extension"]["name"].as_str().unwrap(), "pg_cron");
        //   assert_eq!(table["extension"]["version"].as_str().unwrap(), "1.5.2");
        //   assert_eq!(
        //       table["extension"]["repository"].as_str().unwrap(),
        //       "https://github.com/citusdata/pg_cron"
        //   );
        //   assert_eq!(
        //       table["extension"]["license"].as_str().unwrap(),
        //       "PostgreSQL"
        //   );
        //   assert_eq!(
        //       table["extension"]["description"].as_str().unwrap(),
        //      "Run periodic jobs in PostgreSQL"
        //  );
        //  assert_eq!(
        //      table["extension"]["homepage"].as_str().unwrap(),
        //      "https://github.com/citusdata/pg_cron"
        //  );
        //  assert_eq!(
        //      table["extension"]["documentation"].as_str().unwrap(),
        //      "https://github.com/citusdata/pg_cron"
        //  );
        //  let categories = table["extension"]["categories"].as_array().unwrap();
        //  let mut v: Vec<String> = Vec::new();
        //  for i in categories {
        //      let s = i.as_str();
        //      let s = s.unwrap().to_string();
        //      v.push(s);
        //  }
        //  assert_eq!(v, vec!("analytics", "debugging"));
        //  assert_eq!(
        //      table["extension"]["registry"].as_str().unwrap(),
        //      "https://my.dummy.registry.dev"
        //  )//;

        //  assert_eq!(table["build"]["dockerfile"].as_str().unwrap(), "Dockerfile");
        //  assert_eq!(
        //      table["build"]["install_command"].as_str().unwrap(),
        //      "cd pg_cron && make install"
        //  );
    }

    #[test]
    fn test_parse_trunk_toml_invalid() {
        let toml = "this is not valid toml";
        let result = parse_trunk_toml(toml.as_bytes());
        assert!(result.is_err());
    }
}
