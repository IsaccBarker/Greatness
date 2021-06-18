use snafu::{ResultExt, Snafu};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::result::Result;

use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum ManifestError {
    #[snafu(display("Could not create great file/directory ({}): {}", filename.display(), source))]
    CreateManifest {
        filename: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Could not open great manifest file ({}): {}", filename.display(), source))]
    OpenManifest {
        filename: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Could not read great manifest file ({}): {}", filename.display(), source))]
    ReadManifest {
        filename: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Could not write to great manifest file ({}): {}", filename.display(), source))]
    WriteManifest {
        filename: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Could not parse great configuration file ({}): {}", filename.display(), source))]
    ParseError {
        filename: PathBuf,
        source: serde_yaml::Error,
    },
    /* #[snafu(display("File {} does not have the correct great permisions: {}", filename.display(), source))]
    PermsError {
        filename: PathBuf,
        source: std::io::Error,
    } */
}

#[derive(Debug, PartialEq)]
pub struct Manifest {
    pub greatness_dir: PathBuf,
    pub greatness_files_dir: PathBuf,
    pub greatness_manifest: PathBuf,

    pub data: ManifestData,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ManifestData {
    pub files: Option<Vec<(PathBuf, PathBuf)>>,
    pub requires: Option<Vec<String>>,
}

impl ManifestData {
    pub fn populate_from_file(
        manifest_info: &Manifest,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let manifest_file = &manifest_info.greatness_manifest;
        let x = serde_yaml::from_str(&fs::read_to_string(&manifest_file).context(
            super::manifest::ReadManifest {
                filename: &manifest_file,
            },
        )?)
        .context(ParseError {
            filename: manifest_file,
        })?;

        Ok(x)
    }

    pub fn populate_file(&self, manifest: &Manifest) {
        let mut s = serde_yaml::to_string(self).unwrap(); // TODO: Figure out if we actually have to worry about this
        let mut f = File::create(&manifest.greatness_manifest).unwrap();
        f.set_len(0).unwrap(); // Erase the file;
        f.write_all(unsafe { s.as_mut_vec() }).unwrap();
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
    pub fn new() -> Result<Self, ManifestError> {
        let home_dir = home::home_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
        let mut greatness_dir = PathBuf::from(home_dir.clone());
        greatness_dir.push(".greatness");
        let mut greatness_files_dir = PathBuf::from(greatness_dir.clone());
        greatness_files_dir.push("files");
        let mut greatness_manifest = PathBuf::from(greatness_dir.clone());
        greatness_manifest.push("greatness.yaml");

        debug!(
            "Working the greatest directory of {}!",
            greatness_dir.display()
        );

        if !greatness_dir.as_path().exists() {
            fs::create_dir(&greatness_dir).context(CreateManifest {
                filename: &greatness_dir,
            })?;
        }

        if !greatness_files_dir.as_path().exists() {
            fs::create_dir(&greatness_files_dir).context(CreateManifest {
                filename: &greatness_files_dir,
            })?;
        }

        let mut file;
        if !greatness_manifest.as_path().exists() {
            file = File::create(&greatness_manifest).context(CreateManifest {
                filename: &greatness_manifest,
            })?;

            file.write_all(b"{}").context(WriteManifest {
                // For some reason, we need non-valid YAML to not get an error
                filename: &greatness_manifest,
            })?;
        }

        Ok(Self {
            greatness_dir,
            greatness_files_dir,
            greatness_manifest,
            data: ManifestData::default(),
        })
    }
}
