use crate::manifest::State;
use crate::utils;
use clap::ArgMatches;
use log::warn;

pub fn rm(matches: &ArgMatches, manifest: &mut State) -> Result<(), utils::CommonErrors> {
    for unwanted_package in matches.values_of("packages").unwrap() {
        if manifest.data.contains_package(unwanted_package.to_owned()).is_none() {
            warn!("Not-yet-great package {} is not yet added! Skipping....", unwanted_package);
        }
    }

    if let Some(packages) = &mut manifest.data.packages {
        packages.retain(|e| {
            for unwanted_package in matches.values_of("packages").unwrap() {
                if e.package == unwanted_package {
                    return false;
                }
            }

            true
        });
    }

    manifest.data.populate_file(manifest);

    Ok(()) 
}

