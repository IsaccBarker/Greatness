use crate::manifest::State;
use clap::ArgMatches;

pub fn rm(matches: &ArgMatches, state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(packages) = &mut state.data.packages {
        for package in packages {
            for overload in matches.values_of("overloads").unwrap() {
                package.package_overloads.retain(|_, v| v != overload);
            }
        }
    }

    state.data.populate_file(state);

    Ok(())
}
