use crate::manifest::Manifest;
use crate::utils;
use clap::ArgMatches;
use snafu::ResultExt;
use std::path::PathBuf;

pub fn jog(matches: &ArgMatches, manifest: &mut Manifest) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(files) = &manifest.data.files {
        for file in files {
            if let Some(scripts) = &file.scripts {
                for script in scripts {
                    manifest.script_state.script_on_file(&utils::special_to_absolute(&file.path), &utils::special_to_absolute(script))?;
                }
            }
        }
    }

    Ok(())
}
