use anyhow::Error;
use ignore::Error as IgnoreError;
use std::{path::Path};
use std::path::PathBuf;

pub fn find_licenses(
    directory: &Path,
) -> Result<Vec<PathBuf>, Error> {
    use ignore::types::TypesBuilder;
    use ignore::WalkBuilder;

    let mut types_builder = TypesBuilder::new();
    types_builder.add_defaults();
    types_builder.select("license");
    let matcher = types_builder.build().unwrap();

    let mut paths = Vec::new();
    WalkBuilder::new(directory)
        .types(matcher)
        .build()
        .filter_map(|entry| match entry {
            Ok(entry) => Some(entry),
            Err(error) => {
                IgnoreError::WithPath { path: Default::default(), err: Box::from(error) };
                None
            }
        })
        .filter(|entry| !entry.metadata().unwrap().is_dir())
        .for_each(|entry| {
            let path = entry.path();
            let path_owned = path.to_owned();
            paths.push(path_owned);
        });

    Ok(paths)
}
