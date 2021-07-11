use crate::manifest::State;
use crate::utils;
use clap::ArgMatches;
use log::warn;

pub fn rm(matches: &ArgMatches, state: &mut State) -> Result<(), utils::CommonErrors> {
    for unwanted_package in matches.values_of("packages").unwrap() {
        if state
            .data
            .contains_package(unwanted_package.to_owned())
            .is_none()
        {
            warn!(
                "Not-yet-great package {} is not yet added! Skipping....",
                unwanted_package
            );
        }
    }

    if let Some(packages) = &mut state.data.packages {
        packages.retain(|e| {
            for unwanted_package in matches.values_of("packages").unwrap() {
                if e.package == unwanted_package {
                    return false;
                }
            }

            true
        });
    }

    state.data.populate_file(state);

    Ok(())
}
