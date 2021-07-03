use crate::manifest::Manifest;
use crate::utils;
use clap::ArgMatches;
use std::path::PathBuf;

/// Given a file and a script, remove the script
/// from the file.
pub fn rm(matches: &ArgMatches, manifest: &mut Manifest) -> Result<(), Box<dyn std::error::Error>> {
    let target_script = PathBuf::from(matches.value_of("script").unwrap());
    let target_file = PathBuf::from(matches.value_of("file").unwrap());
    let specified_script = utils::relative_to_special(&utils::relative_to_script(
        manifest,
        &PathBuf::from(target_script),
    ))?;
    let specified_file = utils::relative_to_special(&target_file)?;

    // Like a birds nest. Fix?
    if let Some(files) = &mut manifest.data.files {
        for file in files {
            if file.path == specified_file {
                if let Some(scripts) = &mut file.scripts {
                    scripts.retain(|e| {
                        println!("{} == {}", e.display(), &specified_script.display());
                        e != &specified_script
                    });

                    if scripts.len() == 0 {
                        file.scripts = None;
                    }
                }
            }
        }
    }

    manifest.data.populate_file(&manifest);

    Ok(())
}
