use std::path::PathBuf;
use crate::utils;
use crate::manifest::Manifest;
use clap::ArgMatches;
use snafu::ResultExt;

pub fn rm(matches: &ArgMatches, manifest: &mut Manifest) -> Result<(), utils::CommonErrors> {
    let files = matches.values_of("files").unwrap();

    for file in files.into_iter() {
        let data = manifest.data.clone();
        let contains = data.contains(&utils::absolute_to_special(&PathBuf::from(file).canonicalize().unwrap()));
        if contains.is_none() {
            Err(std::io::Error::from(std::io::ErrorKind::NotFound)).context(utils::FileNotTracked{file})?;
        }

        let mut added_files = manifest.data.files.take().unwrap_or(vec![]);
        added_files.remove(contains.unwrap().1);
        manifest.data.files.replace(added_files);
    }

    manifest.data.populate_file(manifest);

    Ok(())
}
