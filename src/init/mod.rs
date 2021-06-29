use crate::manifest::Manifest;
use crate::utils;
use clap::ArgMatches;
use snafu::ResultExt;
use std::fs;
use std::fs::File;
use std::io::Write;

/// Initializes the environment and gets it ready for greatness.
/// WILL OVERWRITE THE USERS CONFIGURATION!!! However, it will
/// not delete any dotfiles themselves.
pub fn init(_matches: &ArgMatches, manifest: &Manifest) -> Result<(), Box<dyn std::error::Error>> {
    if !manifest.greatness_dir.as_path().exists() {
        fs::create_dir(&manifest.greatness_dir).context(utils::DirCreationError {
            dir: &manifest.greatness_dir,
        })?;
    }

    if !manifest.greatness_pulled_dir.as_path().exists() {
        fs::create_dir(&manifest.greatness_pulled_dir).context(utils::DirCreationError {
            dir: &manifest.greatness_pulled_dir,
        })?;
    }

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
