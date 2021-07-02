use crate::manifest::{AddedFile, Manifest};
use crate::utils;
use clap::ArgMatches;
use log::debug;
use snafu::ResultExt;
use std::path::PathBuf;

/// Assigns a script to a file
pub fn assign(
    matches: &ArgMatches,
    manifest: &mut Manifest,
) -> Result<(), Box<dyn std::error::Error>> {
    let target_base = PathBuf::from(matches.value_of("file").unwrap());
    let mut script_base = manifest.greatness_scripts_dir.clone();
    script_base.push(PathBuf::from(matches.value_of("script").unwrap()));

    debug!("Assigning script at {} to {} (non special paths)....", script_base.display(), target_base.display());

    let target = utils::absolute_to_special(
        &PathBuf::from(&target_base)
            .canonicalize()
            .context(utils::NoFileExistsError { file: target_base })?,
    );
    let script = utils::absolute_to_special(
        &PathBuf::from(&script_base)
            .canonicalize()
            .context(utils::NoFileExistsError { file: script_base })?,
    );

    if manifest.data.contains(&target).is_none() {
        return Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
            .context(utils::FileNotTracked { file: target })?;
    }

    if let Some(files) = &mut manifest.data.files {
        for file in files {
            if file.path == target {
                if let Some(scripts) = &mut file.scripts {
                    if !scripts.contains(&script) {
                        scripts.push(script);
                    }

                    break;
                } else {
                    file.scripts = Some(vec![script]);

                    break;
                }
            }
        }
    }

    manifest.data.populate_file(&manifest);

    Ok(())
}
