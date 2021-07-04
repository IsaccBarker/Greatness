// use std::path::PathBuf;
use crate::manifest::Manifest;
use crate::utils;
use clap::ArgMatches;
// use snafu::ResultExt;

pub fn repel(matches: &ArgMatches, manifest: &mut Manifest) -> Result<(), utils::CommonErrors> {
    let to_repel = matches.value_of("name").unwrap();

    if let Some(requires) = &mut manifest.data.requires {
        requires.retain(|e| {
            e.1.components()
                .last()
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap()
                != to_repel
        });
    }

    manifest.data.populate_file(&manifest);

    Ok(())
}
