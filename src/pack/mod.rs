use crate::manifest::State;
use crate::utils;
use clap::ArgMatches;
use log::debug;
use snafu::{ResultExt, Snafu};
use std::path::PathBuf;

#[derive(Debug, Snafu)]
pub enum PackError {
    #[snafu(display("Invalid great pack type: {}: {}", pack_type, source))]
    PackType {
        pack_type: String,
        source: std::io::Error,
    },
}

/// Pack, and automatically call a packing backend
pub fn pack(manifest: &mut State, _matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let original_manifest_location = manifest.greatness_manifest.clone();
    let base = PathBuf::from(&manifest.greatness_git_pack_dir);
    manifest.greatness_manifest = base.clone();
    if !base.as_path().exists() {
        std::fs::create_dir(&base).context(utils::DirCreationError { dir: &base })?;
    }

    manifest.greatness_manifest.push("greatness.yaml");
    if !manifest.greatness_manifest.as_path().exists() {
        std::fs::File::create(&manifest.greatness_manifest).context(utils::FileCreationError {
            file: &manifest.greatness_manifest,
        })?;
    }

    pack_manifest(manifest, &original_manifest_location)?;
    pack_files(manifest, &base)?;

    Ok(())
}

/// Pack the manifest. Thats it.
fn pack_manifest(
    manifest: &mut State,
    original_manifest_location: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // Pack the manifest
    debug!(
        "Packing manifest from {} -> {}....",
        original_manifest_location.display(),
        &manifest.greatness_manifest.display()
    );
    std::fs::copy(&original_manifest_location, &manifest.greatness_manifest).context(
        utils::FileCopyError {
            src: &original_manifest_location,
            dest: &manifest.greatness_manifest,
        },
    )?;

    Ok(())
}

/// Packs all the files.
pub fn pack_files(manifest: &State, base: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(files) = &manifest.data.files {
        for file in files {
            pack_file(&base, &file.path)?;
        }
    }

    Ok(())
}

/// Pack a file, git style
fn pack_file(base: &PathBuf, file: &PathBuf) -> Result<(), utils::CommonErrors> {
    let absolute_file = utils::special_to_absolute(file);
    let mut to = base.clone();
    to.push("files");
    to.push(utils::absolute_to_special(&absolute_file));

    debug!(
        "Packing file from {} -> {}....",
        &absolute_file.display(),
        &to.display()
    );
    if !to.as_path().exists() {
        std::fs::create_dir_all(to.parent().unwrap()).context(utils::DirCreationError {
            dir: to.parent().unwrap(),
        })?;
    }

    std::fs::copy(&absolute_file, &to).context(utils::FileCopyError {
        src: &absolute_file,
        dest: &to,
    })?;

    Ok(())
}
