use crate::manifest::State;
use crate::utils;
use clap::ArgMatches;
use git2::Repository;
use snafu::{ResultExt, Snafu};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Snafu)]
pub enum InitError {
    #[snafu(display("Could not initialize repository (at {}) for packaging: {}", dir.display(), source))]
    NoRepoInit { dir: PathBuf, source: git2::Error },
}

/// Initializes the environment and gets it ready for greatness.
/// WILL OVERWRITE THE USERS CONFIGURATION!!! However, it will
/// not delete any dotfiles themselves.
pub fn init(_matches: &ArgMatches, state: &State) -> Result<(), Box<dyn std::error::Error>> {
    init_no_damage(_matches, state)?;

    let mut file;
    file = File::create(&state.greatness_state).context(utils::FileCreationError {
        file: &state.greatness_state,
    })?;

    // For some reason, we need non-valid YAML to not get an error
    file.write_all(b"{}").context(utils::FileWriteError {
        file: &state.greatness_state,
    })?;

    Repository::init(&state.greatness_git_pack_dir).context(NoRepoInit {
        dir: &state.greatness_git_pack_dir,
    })?;

    Ok(())
}

/// Initialize, but don't damage anything
pub fn init_no_damage(
    _matches: &ArgMatches,
    state: &State,
) -> Result<(), Box<dyn std::error::Error>> {
    if !state.greatness_dir.as_path().exists() {
        fs::create_dir_all(&state.greatness_dir).context(utils::DirCreationError {
            dir: &state.greatness_dir,
        })?;
    }

    if !state.greatness_pulled_dir.as_path().exists() {
        fs::create_dir_all(&state.greatness_pulled_dir).context(utils::DirCreationError {
            dir: &state.greatness_pulled_dir,
        })?;
    }

    if !state.greatness_git_pack_dir.as_path().exists() {
        fs::create_dir_all(&state.greatness_git_pack_dir).context(utils::DirCreationError {
            dir: &state.greatness_git_pack_dir,
        })?;
    }

    if !state.greatness_scripts_dir.as_path().exists() {
        fs::create_dir_all(&state.greatness_scripts_dir).context(utils::DirCreationError {
            dir: &state.greatness_scripts_dir,
        })?;
    }

    Ok(())
}
