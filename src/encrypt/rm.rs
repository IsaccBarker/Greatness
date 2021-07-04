use crate::manifest::Manifest;
use crate::utils;
use clap::ArgMatches;
use snafu::ResultExt;
use std::path::PathBuf;

pub fn rm(matches: &ArgMatches, manifest: &mut Manifest) -> Result<(), Box<dyn std::error::Error>> {
    for file in matches.values_of("files").unwrap() {
        if let Some(manifest_files) = &mut manifest.data.files {
            for manifest_file in manifest_files {
                if manifest_file.path
                    == utils::relative_to_special(&PathBuf::from(file))
                        .context(utils::NoFileExistsError { file })?
                {
                    manifest_file.encrypted = false;
                }
            }
        }
    }

    manifest.data.populate_file(manifest);

    Ok(())
}
