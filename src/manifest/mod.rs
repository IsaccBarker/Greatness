use crate::script::ScriptsState;
use crate::utils;
use git2::Repository;
use snafu::{ResultExt, Snafu};
use std::convert::From;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::result::Result;

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

/// Contains local data on disk and paths got dynamically.
/// For any new entry, please add to the status code.
pub struct Manifest {
    pub greatness_dir: PathBuf,
    pub greatness_pulled_dir: PathBuf,
    pub greatness_manifest: PathBuf,
    pub greatness_git_pack_dir: PathBuf,
    pub greatness_scripts_dir: PathBuf,
    pub repository: Option<Repository>,
    pub script_state: ScriptsState,

    pub data: ManifestData,
}

/// Contains information about an added file.
/// For any new entry, please add to the status code.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AddedFile {
    #[serde(default)]
    pub path: PathBuf,
    #[serde(default)]
    pub tag: Option<String>,
    #[serde(default)]
    pub scripts: Option<Vec<PathBuf>>,
    #[serde(default)]
    pub encrypted: bool,
}

/// Data stored in the manifest that is stored locally on the computer
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ManifestData {
    /// Dot files
    pub files: Option<Vec<AddedFile>>,
    /// Required repositories of dotfiles. First element is an optional URL
    /// in which to update, and the second is a required path on the local disk.
    pub requires: Option<Vec<(Option<String>, PathBuf)>>,
}

impl From<PathBuf> for AddedFile {
    fn from(path: PathBuf) -> Self {
        Self {
            path,
            tag: Some("".to_owned()),
            scripts: None,
            encrypted: false,
        }
    }
}

impl From<(PathBuf, String)> for AddedFile {
    fn from((path, tag): (PathBuf, String)) -> Self {
        Self {
            path,
            tag: Some(tag),
            scripts: None,
            encrypted: false,
        }
    }
}

impl Default for AddedFile {
    fn default() -> Self {
        Self {
            path: PathBuf::from(""),
            tag: None,
            scripts: None,
            encrypted: false,
        }
    }
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

        debug!("Writing to file:\n{}", s);

        f.set_len(0).unwrap(); // Erase the file;
        f.write_all(unsafe { s.as_mut_vec() }).unwrap();
    }

    /// Detects if we already contain a file
    /// It returns the matching element, and its index.
    pub fn contains(&self, looking_for: &PathBuf) -> Option<(&AddedFile, usize)> {
        // BUG: ALLWAYS RETURNS 0 FOR THE INDEX.
        let looking_for_normalized = looking_for;
        if let Some(files) = &self.files {
            let mut i: usize = 0;
            for file in files {
                if looking_for_normalized
                    .clone()
                    .into_os_string()
                    .to_str()
                    .unwrap()
                    .to_string()
                    == file.clone().path.into_os_string().into_string().unwrap()
                {
                    return Some((file, i));
                }

                i += 1;
            }
        }

        None
    }

    /// Adds a file. Will not add if the file
    /// it is not unique
    pub fn add_file(&mut self, file: AddedFile) {
        let mut files = self.files.take().unwrap_or(vec![]);
        self.files.replace(files.clone());
        let contains = self.contains(&file.path);

        if contains.is_some() {
            files.remove(contains.unwrap().1);
        }

        if contains.is_some() {
            files.push(file);
        } else {
            files = vec![file];
        }

        self.files.replace(files);
    }

    /// Gets all the tags in use
    pub fn all_tags(&self) -> Option<Vec<String>> {
        let mut tags = vec![];
        if self.files.is_none() {
            return None;
        }

        for file in self.files.clone().unwrap() {
            if file.tag.is_none() {
                continue;
            }

            tags.push(file.tag.unwrap());
        }

        Some(tags)
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
        let mut greatness_git_pack_dir = PathBuf::from(manifest_dir.clone());
        greatness_git_pack_dir.push("packed");
        greatness_git_pack_dir.push("git");
        let mut greatness_scripts_dir = PathBuf::from(manifest_dir.clone());
        greatness_scripts_dir.push("scripts");

        let mut script_state = ScriptsState::new();
        script_state.register_all();

        let mut repository: Option<Repository> = None;

        if greatness_git_pack_dir.exists() {
            repository = Some(Repository::open(&greatness_git_pack_dir).unwrap());
        }

        debug!(
            "Working the greatest directory of {}!",
            manifest_dir.display()
        );

        Ok(Self {
            greatness_dir: manifest_dir,
            greatness_manifest,
            greatness_pulled_dir,
            greatness_git_pack_dir,
            greatness_scripts_dir,
            repository,
            script_state,
            data: ManifestData::default(),
        })
    }
}
