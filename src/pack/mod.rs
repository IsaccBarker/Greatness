use crate::manifest::State;
use crate::utils;
use clap::ArgMatches;
use log::debug;
use snafu::{ResultExt, Snafu};
use std::path::PathBuf;

#[derive(Debug, Snafu)]
pub enum PackError {
    #[snafu(display("Invalid great pack type: {}: {}", pack_type, source))]
    PackType {
        pack_type: String,
        source: std::io::Error,
    },
}

/// Pack, and automatically call a packing backend
pub fn pack(state: &mut State, _matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let original_state_location = state.greatness_state.clone();
    let base = PathBuf::from(&state.greatness_git_pack_dir);
    state.greatness_state = base.clone();
    if !base.as_path().exists() {
        std::fs::create_dir(&base).context(utils::DirCreationError { dir: &base })?;
    }

    state.greatness_state.push("greatness.yaml");
    if !state.greatness_state.as_path().exists() {
        std::fs::File::create(&state.greatness_state).context(utils::FileCreationError {
            file: &state.greatness_state,
        })?;
    }

    pack_state(state, &original_state_location)?;
    pack_files(state, &base)?;
    pack_scripts(state, &base)?;

    Ok(())
}

/// Pack the state. Thats it.
fn pack_state(
    state: &mut State,
    original_state_location: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // Pack the state
    debug!(
        "Packing state from {} -> {}....",
        original_state_location.display(),
        &state.greatness_state.display()
    );
    std::fs::copy(&original_state_location, &state.greatness_state).context(
        utils::FileCopyError {
            src: &original_state_location,
            dest: &state.greatness_state,
        },
    )?;

    Ok(())
}

/// Packs the scripts
fn pack_scripts(state: &mut State, base: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut to = base.clone();
    to.push("scripts");

    if !to.as_path().exists() {
        debug!(
            "Scripts dir doesn't exist! Creating at {}....",
            &to.display()
        );
        std::fs::create_dir_all(&to).context(utils::DirCreationError { dir: &to })?;
    }

    debug!(
        "Packing script directory from {} -> {}....",
        state.greatness_scripts_dir.display(),
        to.display(),
    );

    // Copy the directory recursively
    fs_extra::dir::copy(
        &state.greatness_scripts_dir,
        to.parent().unwrap(),
        &fs_extra::dir::CopyOptions::new(),
    )?;

    Ok(())
}

/// Packs all the files.
pub fn pack_files(state: &State, base: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(files) = &state.data.files {
        for file in files {
            pack_file(&base, &file.path)?;
        }
    }

    Ok(())
}

/// Pack a file, git style
fn pack_file(base: &PathBuf, file: &PathBuf) -> Result<(), utils::CommonErrors> {
    let absolute_file = utils::special_to_absolute(file);
    let mut to = base.clone();
    to.push("files");
    to.push(utils::absolute_to_special(&absolute_file));

    debug!(
        "Packing file from {} -> {}....",
        &absolute_file.display(),
        &to.display()
    );
    if !to.as_path().exists() {
        debug!("Files dir doesn't exist! Creating at {}....", &to.display());
        std::fs::create_dir_all(to.parent().unwrap()).context(utils::DirCreationError {
            dir: to.parent().unwrap(),
        })?;
    }

    std::fs::copy(&absolute_file, &to).context(utils::FileCopyError {
        src: &absolute_file,
        dest: &to,
    })?;

    Ok(())
}
