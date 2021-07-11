use crate::manifest::{AddedPackage, State};
use clap::ArgMatches;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum AddPackageError {
    #[snafu(display(
        "Great package {} must be added before an overload can be specified!",
        package
    ))]
    PackageNotAdded {
        package: String,
        source: std::io::Error,
    },

    #[snafu(display("No such great manager found for {}!", manager))]
    NoSuchManager {
        manager: String,
        source: std::io::Error,
    },

    #[snafu(display(
        "Invalid overload input {}. Please specify as manager:original:overload!",
        input
    ))]
    InvalidOverloadInput {
        input: String,
        source: std::io::Error,
    },
}

pub fn add(matches: &ArgMatches, state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    for package in matches.values_of("packages").unwrap() {
        let mut added = AddedPackage::new();

        added.package = package.to_string();

        if let Some(packages) = &mut state.data.packages {
            packages.push(added);
        } else {
            state.data.packages = Some(vec![added]);
        }
    }

    state.data.populate_file(&state);

    Ok(())
}
