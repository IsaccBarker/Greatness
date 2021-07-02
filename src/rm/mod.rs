use crate::manifest::Manifest;
use crate::utils;
use clap::ArgMatches;
use snafu::ResultExt;
use std::path::PathBuf;

pub fn rm(matches: &ArgMatches, manifest: &mut Manifest) -> Result<(), utils::CommonErrors> {
    let files = matches.values_of("files").unwrap();

    for file in files.into_iter() {
        // We cannot canonicalize path if it doesn't exist, so we create it temporalily.
        let mut must_delete_tmp = false;
        if !PathBuf::from(file).exists() {
            std::fs::File::create(file).context(utils::FileCreationError { file })?;
            must_delete_tmp = true;
        }

        let data = manifest.data.clone();
        let contains = data.contains(&utils::relative_to_special(
            &PathBuf::from(file),
        ).unwrap());
        if contains.is_none() {
            Err(std::io::Error::from(std::io::ErrorKind::NotFound))
                .context(utils::FileNotTracked { file })?;
        }

        let mut added_files = manifest.data.files.take().unwrap_or(vec![]);
        added_files.remove(contains.unwrap().1);
        manifest.data.files.replace(added_files);

        // Delete said temporary file if it exists
        if must_delete_tmp {
            std::fs::remove_file(file).context(utils::FileDeletionError { file })?;
        }
    }

    manifest.data.populate_file(manifest);

    Ok(())
}
