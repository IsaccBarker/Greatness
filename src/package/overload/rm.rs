use crate::manifest::State;
use clap::ArgMatches;

pub fn rm(matches: &ArgMatches, manifest: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(packages) = &mut manifest.data.packages {
        for package in packages {
            for overload in matches.values_of("overloads").unwrap() {
                package.package_overloads.retain(|_, v| v != overload);
            }
        }
    }

    manifest.data.populate_file(manifest);

    Ok(())
}
