use git2::Repository;
use crate::manifest::Manifest;
use crate::utils;
use clap::ArgMatches;
use snafu::{Snafu, ResultExt};
use std::fs;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Snafu)]
pub enum InitError {
    #[snafu(display("Could not initialize repository (at {}) for packaging: {}", dir.display(), source))]
    NoRepoInit {
        dir: PathBuf,
        source: git2::Error
    }
}

/// Initializes the environment and gets it ready for greatness.
/// WILL OVERWRITE THE USERS CONFIGURATION!!! However, it will
/// not delete any dotfiles themselves.
pub fn init(_matches: &ArgMatches, manifest: &Manifest) -> Result<(), Box<dyn std::error::Error>> {
    if !manifest.greatness_dir.as_path().exists() {
        fs::create_dir_all(&manifest.greatness_dir).context(utils::DirCreationError {
            dir: &manifest.greatness_dir,
        })?;
    }

    if !manifest.greatness_pulled_dir.as_path().exists() {
        fs::create_dir_all(&manifest.greatness_pulled_dir).context(utils::DirCreationError {
            dir: &manifest.greatness_pulled_dir,
        })?;
    }

    if !manifest.greatness_git_pack_dir.as_path().exists() {
        fs::create_dir_all(&manifest.greatness_git_pack_dir).context(utils::DirCreationError {
            dir: &manifest.greatness_git_pack_dir,
        })?;
    }

    Repository::init(&manifest.greatness_git_pack_dir).context(NoRepoInit{dir: &manifest.greatness_git_pack_dir})?;

    let mut file;
    file = File::create(&manifest.greatness_manifest).context(utils::FileCreationError {
        file: &manifest.greatness_manifest,
    })?;

    // For some reason, we need non-valid YAML to not get an error
    file.write_all(b"{}").context(utils::FileWriteError {
        file: &manifest.greatness_manifest,
    })?;

    Ok(())
}
