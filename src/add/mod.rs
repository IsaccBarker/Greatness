use clap::ArgMatches;
use snafu::Snafu;

use crate::manifest::{AddedFile, State};
use crate::utils;
use log::{debug, info, warn};
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
    state_info: &mut State,
) -> Result<(), Box<dyn std::error::Error>> {
    // Do some checks to figure out which files we should include
    // println!("{:?}", files);
    only_retain_correct_files(&mut files, state_info);

    for file in files.iter() {
        debug!("Adding file {}....", file.display());
        add_file(&PathBuf::from(file), state_info, matches)?;
    }

    state_info.data.populate_file(state_info);

    Ok(())
}

fn only_retain_correct_files(files: &mut Vec<PathBuf>, state_info: &mut State) {
    files.retain(|file| {
        if !std::path::Path::new(file).is_file() {
            info!(
                "The file {} doesn't exist, and thus cannot become great (be added). Skipping....",
                file.display()
            );
            return false;
        }

        if state_info
            .data
            .contains(&utils::relative_to_special(&file).unwrap())
            .is_some()
        {
            warn!(
                "The file {} is already great (already added)! Skipping....",
                file.display()
            );

            return false;
        }

        if file.symlink_metadata().unwrap().file_type().is_symlink() {
            warn!(
                "The file {} is a symlink. Greatness cannot handle symlinks. Skipping....",
                file.display()
            );

            return false;
        }

        return true; // Keep the element
    });
}

fn add_file(
    file: &PathBuf,
    state: &mut State,
    _matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let special_file = utils::relative_to_special(&file)?;
    if let Some(ref mut files) = state.data.files {
        files.push(AddedFile::from(special_file.clone()));
    } else {
        state.data.files = Some(vec![AddedFile::from(special_file.clone())]);
    }

    Ok(())
}
