use crate::manifest::AddedFile;
use crate::manifest::State;
use crate::utils;
use clap::ArgMatches;
use log::{debug, warn};
use snafu::ResultExt;
use std::path::PathBuf;

/// Assigns a script to a file
pub fn assign(matches: &ArgMatches, state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    let target_base = PathBuf::from(matches.value_of("file").unwrap());
    let script_base =
        utils::relative_to_script(state, &PathBuf::from(matches.value_of("script").unwrap()));

    debug!(
        "Assigning script at {} to {} (non special paths)....",
        script_base.display(),
        target_base.display()
    );

    let target = utils::relative_to_special(&PathBuf::from(&target_base))?;
    let script = utils::absolute_to_special(
        &PathBuf::from(&script_base)
            .canonicalize()
            .context(utils::NoFileExistsError { file: script_base })?,
    );

    if state.data.contains(&target).is_none() {
        return Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
            .context(utils::FileNotTracked { file: target })?;
    }

    if let Some(files) = &mut state.data.files {
        for file in files {
            if assign_file(file, script.clone(), &target) {
                break;
            }
        }
    }

    state.data.populate_file(&state);

    Ok(())
}

fn assign_file(file: &mut AddedFile, script: PathBuf, target: &PathBuf) -> bool {
    if &file.path == target {
        if let Some(scripts) = &mut file.scripts {
            if !scripts.contains(&script) {
                scripts.push(script);
            } else {
                warn!(
                    "The script {} is already assigned/associated with this file! Skipping....",
                    script.display()
                );
            }

            return true;
        } else {
            file.scripts = Some(vec![script]);

            return true;
        }
    }

    false
}
