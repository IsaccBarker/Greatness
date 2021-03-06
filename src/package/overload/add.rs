use crate::manifest::State;
use clap::ArgMatches;
use snafu::{ResultExt, Snafu};

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
}

pub fn add(matches: &ArgMatches, state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    let manager = matches.value_of("manager").unwrap().to_owned();
    let original = matches.value_of("original").unwrap().to_owned();
    let overload = matches.value_of("overload").unwrap().to_owned();

    if !state
        .package_context
        .package_install_prefix
        .contains_key(&manager)
    {
        Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)).context(NoSuchManager {
            manager: manager.clone(),
        })?;
    }

    if let Some(already) = state.data.contains_package(original.to_string()) {
        already.package_overloads.insert(manager, overload);
    } else {
        Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
            .context(PackageNotAdded { package: original })?;
    }

    state.data.populate_file(state);

    Ok(())
}
