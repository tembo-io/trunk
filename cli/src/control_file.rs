use std::ops::Not;

/// Some information contained in an extension's `control` file.
///
/// Postgres docs: https://www.postgresql.org/docs/current/extend-extensions.html
#[derive(Debug)]
pub struct ControlFile {
    pub directory: Option<String>,
    pub module_pathname: Option<String>,
    pub requires: Option<Vec<String>>,
}

impl ControlFile {
    pub fn parse(input: &str) -> Self {
        let mut directory = None;
        let mut module_pathname = None;
        let mut requires = None;

        for line in input.lines().map(str::trim_start) {
            if let Some(rhs) = line.strip_prefix("directory") {
                let value = strip_value(rhs);
                directory = Some(value.to_string());
                continue;
            }

            if let Some(rhs) = line.strip_prefix("module_pathname") {
                let value = strip_value(rhs);
                module_pathname = Some(value.to_string());
                continue;
            }

            if let Some(rhs) = line.strip_prefix("requires") {
                let value = strip_value(rhs);
                requires = Some(
                    value
                        .split(',')
                        .map(str::trim)
                        .filter(|v| v.is_empty().not())
                        .map(ToString::to_string)
                        .collect(),
                );
                continue;
            }
        }

        Self {
            directory,
            module_pathname,
            requires,
        }
    }

    pub fn dependencies(&self) -> &[String] {
        self.requires.as_ref().map(Vec::as_slice).unwrap_or(&[])
    }
}

fn strip_value(input: &str) -> &str {
    let stripped = input.trim_start_matches(|ch| matches!(ch, ' ' | '='));

    let trimmed = stripped.trim_start();
    trimmed.trim_matches('\'')
}

#[cfg(test)]
mod tests {
    use crate::control_file::ControlFile;

    #[test]
    fn parses_directory_field() {
        let contents = r#"
        comment = 'PL/Java procedural language (https://tada.github.io/pljava/)'
        default_version = '1.6.5'
        encoding = UTF8
        directory = 'pljava'
        schema = sqlj
        "#;

        let control_file = ControlFile::parse(contents);

        assert_eq!(control_file.directory.unwrap(), "pljava");
    }

    #[test]
    fn parses_module_pathname_field() {
        let contents = r#"
        comment = 'Distributed message queues'
        default_version = '0.4.2'
        module_pathname = '$libdir/pgmq'
        relocatable = false
        superuser = false
        requires = ''
        "#;

        let control_file = ControlFile::parse(contents);

        assert!(control_file.directory.is_none());
        assert_eq!(control_file.module_pathname.unwrap(), "$libdir/pgmq");
    }

    #[test]
    fn parses_dependencies_field_when_empty() {
        let contents = r#"
        comment = 'Distributed message queues'
        default_version = '0.4.2'
        module_pathname = '$libdir/pgmq'
        relocatable = false
        superuser = false
        requires = ''
"#;

        let control_file = ControlFile::parse(contents);
        assert_eq!(control_file.requires.unwrap(), Vec::<String>::new());
    }

    #[test]
    fn parses_dependency_field() {
        let sample_data = r#"
        comment = 'Distributed message queues'
        default_version = '0.4.2'
        module_pathname = '$libdir/pgmq'
        relocatable = false
        superuser = false
        requires = 'pg_partman, dep2, dep3'
    "#;

        let control_file = ControlFile::parse(sample_data);
        assert_eq!(
            control_file.requires.unwrap(),
            ["pg_partman", "dep2", "dep3"]
        );
    }
}
