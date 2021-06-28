use clap::ArgMatches;
use snafu::Snafu;

use log::{debug, info};
use crate::manifest::Manifest;
use crate::utils;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum TrackError {
    #[snafu(display("Target file {} doesn't exist, but is great nontheless: {}", target.display(), source))]
    TrackedExists {
        target: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Failed to remove original file {}: {}", file.display(), source))]
    TrackedRemoval {
        file: PathBuf,
        source: std::io::Error,
    },
}

pub fn add_files(
    matches: &ArgMatches,
    mut files: Vec<PathBuf>,
    manifest_info: &mut Manifest,
) -> Result<(), Box<dyn std::error::Error>> {

    // Do some checks to figure out which files we should include
    files.retain(|file| {
        if !std::path::Path::new(file).is_file() {
            info!(
                "The file {} doesn't exist, and thus cannot become great (be added). Skipping....",
                file.display());
            return false;
        }

        if manifest_info.data.contains(file) {
            info!(
                "The file {} is already great (already added)! Skipping....",
                file.display());

            return false;
        }

        if file.symlink_metadata().unwrap().file_type().is_symlink() {
            info!(
                "The file {} is a symlink. Greatness cannot handle symlinks. Skipping....",
                file.display());

            return false;
        }

        return true; // Keep the element
    });

    for file in files.iter() {
        debug!("Adding file {}....", file.display());
        add_file(&PathBuf::from(file), manifest_info, matches)?;
    }

    manifest_info.data.populate_file(manifest_info);

    Ok(())
}

fn add_file(
    file: &PathBuf,
    manifest: &mut Manifest,
    _matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let absolute_file = &fs::canonicalize(file).unwrap();
    let special_file;
    special_file = utils::absolute_to_special(absolute_file);
    if let Some(ref mut files) = manifest.data.files {
        files.push(special_file.clone());
    } else {
        manifest.data.files = Some(vec![special_file.clone()]);
    }

    Ok(())
}
