use snafu::{ResultExt, Snafu};
use crate::manifest::Manifest;
use clap::ArgMatches;
use std::path::PathBuf;
use crate::utils;
use log::debug;

#[derive(Debug, Snafu)]
pub enum PackError {
    #[snafu(display("Invalid great pack type: {}: {}", pack_type, source))]
    PackType {
        pack_type: String,
        source: std::io::Error
    },

    #[snafu(display("Unable to create directory at {}: {}", dir.display(), source))]
    DirectoryCreation {
        dir: PathBuf,
        source: std::io::Error
    },

    #[snafu(display("Unable to create file at {}: {}", file.display(), source))]
    FileCreation {
        file: PathBuf,
        source: std::io::Error
    },

    #[snafu(display("Could not update/pack file at {}: {}", file.display(), source))]
    PackFile {
        file: PathBuf,
        source: std::io::Error
    }
}

pub fn pack(pack_type: String, manifest: &mut Manifest, matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match pack_type.as_str() {
        "git" => {
            pack_git(manifest, matches)?;
        },

        _ => {
            Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)).context(PackType{pack_type})?
        }
    }

    Ok(())
}

fn pack_git(manifest: &mut Manifest, matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let original_manifest_location = manifest.greatness_manifest.clone();
    let base = PathBuf::from(matches.value_of("where").unwrap());
    manifest.greatness_manifest = base.clone();
    if ! base.as_path().exists() {
        std::fs::create_dir(&base).context(DirectoryCreation{dir: &base})?;
    }

    manifest.greatness_manifest.push("greatness.yaml");
    if ! manifest.greatness_manifest.as_path().exists() {
            std::fs::File::create(&manifest.greatness_manifest).context(FileCreation{file: &manifest.greatness_manifest})?;
    }

    // Pack the manifest
    debug!("Packing manifest from {} -> {}....", original_manifest_location.display(), &manifest.greatness_manifest.display());
    std::fs::copy(original_manifest_location, &manifest.greatness_manifest).context(PackFile{file: &manifest.greatness_manifest})?;

    // Pack the files mentioned by the manifest
    if let Some(files) = &manifest.data.files {
        for file in files {
            let absolute_file = utils::special_to_absolute(file);
            let mut to = base.clone();
            to.push("files");
            to.push(utils::absolute_to_special(&absolute_file));

            debug!("Packing file from {} -> {}....", &absolute_file.display(), &to.display());
            if ! to.as_path().exists() {
                 std::fs::create_dir_all(to.parent().unwrap()).context(DirectoryCreation{dir: to.parent().unwrap()})?;
            }
            
            std::fs::copy(&absolute_file, &to).context(PackFile{file: &absolute_file})?;
        }
    }

    Ok(())
}

