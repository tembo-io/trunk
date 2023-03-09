use crate::manifest::Manifest;
use flate2::bufread::GzDecoder;
use std::fs::File;
use std::io;
use std::io::{BufReader, Seek};
use std::path::{Path, PathBuf};
use tar::{Archive, EntryType};
use tempfile::tempfile;

pub struct Package {
    path: PathBuf,
    file: File,
}

impl Into<File> for Package {
    fn into(self) -> File {
        self.file
    }
}

impl Package {
    pub fn new(path: PathBuf) -> Result<Self, io::Error> {
        Ok(Self {
            file: File::open(&path)?,
            path,
        })
    }

    pub fn manifest(&mut self) -> Result<Manifest, anyhow::Error> {
        let input = match self
            .path
            .extension()
            .into_iter()
            .filter_map(|s| s.to_str())
            .next()
        {
            Some("gz") => {
                // unzip the archive into a temporary file
                use read_write_pipe::*;
                let gzip = std::mem::replace(&mut self.file, tempfile()?);
                let decoder = GzDecoder::new(BufReader::new(gzip));
                self.file.write_reader(decoder)?;
                self.file.rewind()?;
                &self.file
            }
            Some("tar") => &self.file,
            _ => return Err(anyhow::Error::msg("unknown file type"))?,
        };

        let mut archive = Archive::new(input);

        let mut manifest: Option<Manifest> = None;
        let entries = archive.entries_with_seek()?;
        for entry in entries {
            let entry = entry?;
            let name = entry.path()?;
            if entry.header().entry_type() == EntryType::file()
                && name == Path::new("manifest.json")
            {
                manifest.replace(serde_json::from_reader(entry)?);
            }
        }

        manifest.ok_or(anyhow::Error::msg("no manifest"))
    }

    /// Returns the original (compressed) file
    pub fn original_file(&self) -> Result<File, io::Error> {
        File::open(&self.path)
    }

    pub fn original_file_path(&self) -> &Path {
        &self.path
    }
}
