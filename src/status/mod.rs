use crate::manifest::Manifest;
use crate::utils;
use clap::ArgMatches;
use log::info;
use snafu::ResultExt;
use std::path::PathBuf;

pub fn print_status(manifest: &Manifest) {
    info!(
        "Greatness directory: \x1b[1m{}\x1b[0m",
        manifest.greatness_dir.display()
    );
    info!(
        "Greatness pulling  : \x1b[1m{}\x1b[0m",
        manifest.greatness_pulled_dir.display()
    );
    info!(
        "Greatness manifest : \x1b[1m{}\x1b[0m",
        manifest.greatness_manifest.display()
    );
    info!(
        "Greatness git pack : \x1b[1m{}\x1b[0m",
        manifest.greatness_git_pack_dir.display(),
    );
    info!(
        "Greatness scripts  : \x1b[1m{}\x1b[0m",
        manifest.greatness_scripts_dir.display()
    );

    print!("\n");

    if let Some(files) = &manifest.data.files {
        info!("Added files:");

        for file in files {
            info!(
                "\tpath: {}",
                utils::special_to_absolute(&file.path).display()
            );

            if file.tag.is_some() && file.tag != Some("".to_owned()) {
                info!("\t\ttag : {}", file.tag.clone().unwrap());
            }

            if file.scripts.is_some() {
                info!("\t\tscripts:");
                for script in file.scripts.as_ref().unwrap() {
                    info!("\t\t\t{}", utils::special_to_absolute(script).display());
                }
            }
        }
    } else {
        info!("\x1b[1mNo files added!\x1b[0m");
    }

    if let Some(requires) = &manifest.data.requires {
        info!("\nExternal repositories of dotfiless:");

        for required in requires {
            info!("\tat : {}", required.1.display());

            if required.0.is_some() {
                info!("\turl: {}", required.0.clone().unwrap());
            }
        }
    } else {
        info!("\x1b[1mNo external repositories installed!\x1b[0m");
    }
}

pub fn print_file_status(
    manifest: &Manifest,
    matches: &ArgMatches,
) -> Result<(), utils::CommonErrors> {
    let file =
        utils::relative_to_special(&PathBuf::from(matches.value_of("file").unwrap())).unwrap();
    let contains = manifest.data.contains(&file);
    let contains_unwrapped;

    if contains.is_none() {
        return Err(std::io::Error::from(std::io::ErrorKind::NotFound))
            .context(utils::NoFileExistsError { file });
    } else {
        contains_unwrapped = contains.unwrap().0;
    }

    info!(
        "path: {}",
        utils::special_to_absolute(&contains_unwrapped.path).display()
    );

    if contains_unwrapped.tag.is_some() {
        info!("tag : {}", contains_unwrapped.tag.clone().unwrap());
    }

    Ok(())
}
