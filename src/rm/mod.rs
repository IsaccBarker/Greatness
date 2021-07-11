use crate::manifest::State;
use crate::utils;
use clap::ArgMatches;
use snafu::ResultExt;
use std::path::PathBuf;
use log::warn;

pub fn rm(matches: &ArgMatches, state: &mut State) -> Result<(), utils::CommonErrors> {
    let files = matches.values_of("files").unwrap();

    for file in files.into_iter() {
        rm_file(file, state)?;
    }

    state.data.populate_file(state);

    Ok(())
}

fn rm_file(file: &str, state: &mut State) -> Result<(), utils::CommonErrors> {
    // We cannot canonicalize path if it doesn't exist, so we create it temporalily.
    let mut must_delete_tmp = false;
    if !PathBuf::from(file).exists() {
        std::fs::File::create(file).context(utils::FileCreationError { file })?;
        must_delete_tmp = true;
    }

    let data = state.data.clone();
    let contains = data.contains(&utils::relative_to_special(&PathBuf::from(file)).unwrap());
    if contains.is_none() {
        warn!("Not-yet-great file {} is not tracked! Skipping....", file);
    }

    let mut added_files = state.data.files.take().unwrap_or(vec![]);
    if let Some(c) = &contains {
        added_files.remove(c.1);
        state.data.files.replace(added_files);

        // Delete said temporary file if it exists
        if must_delete_tmp {
            std::fs::remove_file(file).context(utils::FileDeletionError { file })?;
        }
    }

    Ok(())
}
