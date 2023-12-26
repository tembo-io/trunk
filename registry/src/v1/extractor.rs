use anyhow::Context;
use flate2::read::GzDecoder;
use serde::Deserialize;
use std::{
    ffi::OsStr,
    io::{Cursor, Read},
    ops::Not,
};
use tar::EntryType;

use crate::views::extension_publish::{ControlFileMetadata, ExtensionUpload};

use super::repository::ExtensionView;

pub type PostgresVersion = u8;

pub struct ExtractedArchive {
    control_files: Vec<ControlFile>,
    manifest: Manifest,
}

#[derive(Deserialize)]
pub struct Manifest {
    pg_version: PostgresVersion,
}

pub struct ControlFile {
    content: Option<String>,
    extension_name: String,
    dependencies: Option<Vec<String>>,
    default_version: Option<String>,
}

pub fn extract_extension_view(
    tar_gz: &[u8],
    new_extension: &ExtensionUpload,
) -> anyhow::Result<(Vec<ExtensionView>, PostgresVersion)> {
    let ExtractedArchive {
        control_files,
        manifest,
    } = extract_archive(tar_gz)?;

    let mut extension_views: Vec<ExtensionView> = control_files
        .into_iter()
        .map(|control_file| ExtensionView {
            extension_name: control_file.extension_name.clone(),
            version: control_file.default_version.unwrap_or_default(),
            trunk_project_name: new_extension.name.to_string(),
            dependencies_extension_names: control_file.dependencies,
            // TODO: should we clone this for every extension in a Trunk project?
            loadable_libraries: new_extension.libraries.clone(),
            configurations: new_extension.configurations.clone(),
            control_file: Some(ControlFileMetadata {
                absent: false,
                content: control_file.content,
            }),
        })
        .collect();

    // If no control files found, we still want to return extension view information we have available.
    // This includes control_file.absent = true and control_file.content = None
    if extension_views.is_empty() {
        extension_views.push(ExtensionView {
            extension_name: new_extension.extension_name.clone().unwrap_or_default(),
            version: new_extension.vers.to_string(),
            trunk_project_name: new_extension.name.to_string(),
            dependencies_extension_names: None,
            loadable_libraries: new_extension.libraries.clone(),
            configurations: new_extension.configurations.clone(),
            control_file: Some(ControlFileMetadata {
                absent: true,
                content: None,
            }),
        });
    }

    Ok((extension_views, manifest.pg_version))
}

fn extract_archive(tar_gz: &[u8]) -> anyhow::Result<ExtractedArchive> {
    let mut manifest_json = None;
    let mut control_files = vec![];
    let mut buf = Vec::with_capacity(tar_gz.len() * 8);
    GzDecoder::new(tar_gz).read_to_end(&mut buf)?;

    let mut archive = tar::Archive::new(Cursor::new(buf));

    for maybe_entry in archive.entries()? {
        let mut entry = maybe_entry?;
        let header = entry.header();
        let entry_size = header.entry_size().unwrap_or(12500);

        match header.entry_type() {
            EntryType::Regular => {}
            _ => {
                continue;
            }
        }

        let path = entry.path()?;

        match path.extension() {
            Some(ext) if ext == "control" => {
                let extension_name = path
                    .file_stem()
                    .with_context(|| "Control file had no file stem")?
                    .to_string_lossy()
                    .into();

                let cargo_toml_contents = decompress_file(&mut entry, entry_size)?;
                let control_file = parse_control_file(extension_name, cargo_toml_contents);

                control_files.push(control_file);
            }
            Some(ext)
                if ext == "json"
                    && path.file_stem().and_then(OsStr::to_str) == Some("manifest") =>
            {
                // We're now iterating through manifest.json
                let manifest = decompress_file(&mut entry, entry_size)?;
                let manifest: Manifest = serde_json::from_str(&manifest)
                    .with_context(|| "Failed to deserialize manifest.json")?;

                manifest_json = Some(manifest);
            }
            Some(_) | None => continue,
        }
    }

    anyhow::ensure!(
        manifest_json.is_some(),
        "Failed to find manifest.json in archive being uploaded"
    );

    Ok(ExtractedArchive {
        control_files,
        manifest: manifest_json.unwrap(),
    })
}

fn decompress_file(
    entry: &mut tar::Entry<'_, Cursor<Vec<u8>>>,
    entry_size: u64,
) -> anyhow::Result<String> {
    let contents = {
        let mut buf = Vec::with_capacity(entry_size as usize);

        entry.read_to_end(&mut buf)?;
        buf
    };

    String::from_utf8(contents).map_err(Into::into)
}

fn parse_control_file(extension_name: String, control_file: String) -> ControlFile {
    let mut dependencies = Vec::new();
    let mut default_version = None;

    fn strip_value(input: &str) -> &str {
        let stripped = input.trim_start_matches(|ch| matches!(ch, ' ' | '='));

        let trimmed = stripped.trim_start();
        trimmed.trim_matches('\'')
    }

    // Grab all lines from the control file and set as string. None if no lines.
    let control_file_content = if control_file.is_empty() {
        None
    } else {
        Some(control_file.clone())
    };

    for line in control_file.lines() {
        if let Some(rest) = line.strip_prefix("requires") {
            let requires = strip_value(rest)
                .split(',')
                .map(str::trim)
                .filter(|v| v.is_empty().not());

            for dependency in requires {
                dependencies.push(dependency.to_string());
            }
        }

        if let Some(rest) = line.strip_prefix("default_version") {
            let parsed_default_version = strip_value(rest);

            default_version = Some(parsed_default_version.into());
        }
    }

    ControlFile {
        content: control_file_content,
        extension_name,
        dependencies: if dependencies.is_empty() {
            None
        } else {
            Some(dependencies)
        },
        default_version,
    }
}
