use snafu::{ResultExt, Snafu};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::result::Result;
use crate::utils;

use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
/// Errors pretaining to the local manifest
pub enum ManifestError {
    #[snafu(display("Could not parse great configuration file ({}): {}", filename.display(), source))]
    ParseError {
        filename: PathBuf,
        source: serde_yaml::Error,
    },
}

/// Contains local data on disk and paths got dynamically
#[derive(Debug, PartialEq)]
pub struct Manifest {
    pub greatness_dir: PathBuf,
    pub greatness_manifest: PathBuf,
    pub greatness_pulled_dir: PathBuf,

    pub data: ManifestData,
}

/// Data stored in the manifest that is stored locally on the computer
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ManifestData {
    /// Dot files
    pub files: Option<Vec<PathBuf>>,
    /// Required repositories of dotfiles. First element is an optional URL
    /// in which to update, and the second is a required path on the local disk.
    pub requires: Option<Vec<(Option<String>, PathBuf)>>,
}

impl ManifestData {
    /// Load on file data into the manifestData struct.
    pub fn populate_from_file(
        manifest_info: &Manifest,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let manifest_file = &manifest_info.greatness_manifest;
        let x = serde_yaml::from_str(&fs::read_to_string(&manifest_file).context(
            utils::FileReadError {
                file: &manifest_file,
            },
        )?)
        .context(ParseError {
            filename: manifest_file,
        })?;

        Ok(x)
    }

    /// Serialize data back into the local file on disk.
    pub fn populate_file(&self, manifest: &Manifest) {
        let mut s = serde_yaml::to_string(self).unwrap(); // TODO: Figure out if we actually have to worry about this
        let mut f = File::create(&manifest.greatness_manifest).unwrap();
        f.set_len(0).unwrap(); // Erase the file;
        f.write_all(unsafe { s.as_mut_vec() }).unwrap();
    }

    /// Detects if we already contain a file
    /// If I'm not mistaken, this is O(n)?
    pub fn contains(&self, looking_for: &PathBuf) -> bool {
        if let Some(files) = &self.files {
            for file in files {
                if looking_for.clone().canonicalize().unwrap().into_os_string().to_str().unwrap().to_string() ==
                    file.clone().into_os_string().into_string().unwrap() {
                    return true;
                }
            }
        }

        false
    }
}

impl Default for ManifestData {
    fn default() -> Self {
        Self {
            files: None,
            requires: None,
        }
    }
}

impl Manifest {
    /// Creates a new local manifest
    pub fn new(manifest_dir: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let mut greatness_pulled_dir = PathBuf::from(manifest_dir.clone());
        greatness_pulled_dir.push("pulled");
        let mut greatness_manifest = PathBuf::from(manifest_dir.clone());
        greatness_manifest.push("greatness.yaml");

        debug!(
            "Working the greatest directory of {}!",
            manifest_dir.display()
        );

        
        Ok(Self {
            greatness_dir: manifest_dir,
            greatness_manifest,
            greatness_pulled_dir,
            data: ManifestData::default(),
        })
    }
}

