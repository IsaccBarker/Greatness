use crate::manifest::{State, AddedPackage};
use crate::utils;
use clap::ArgMatches;
use snafu::{Snafu, ResultExt};

#[derive(Debug, Snafu)]
pub enum AddPackageError {
    #[snafu(display("Great package {} must be added before an overload can be specified!", package))]
    PackageNotAdded {
        package: String,
        source: std::io::Error,
    },

    #[snafu(display("No such great manager found for {}!", manager))]
    NoSuchManager {
        manager: String,
        source: std::io::Error,
    },

    #[snafu(display("Invalid overload input {}. Please specify as manager:original:overload!", input))]
    InvalidOverloadInput {
        input: String,
        source: std::io::Error,
    }
}

pub fn add(matches: &ArgMatches, manifest: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    for package in matches.values_of("packages").unwrap() {
        let mut added = AddedPackage::new();
        
        if package.contains(":") {
            let manager = package.split(":").collect::<Vec<&str>>().get(0).unwrap().to_string();
            let original = match package.split(":").collect::<Vec<&str>>().get(1) {
                Some(o) => o.to_string(),
                None => Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)).context(InvalidOverloadInput{input: package})?,
            };
            let overload = match package.split(":").collect::<Vec<&str>>().get(2) {
                Some(o) => o.to_string(),
                None => Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)).context(InvalidOverloadInput{input: package})?,
            };

            if !manifest.package_context.package_install_prefix.contains_key(&manager) {
                Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)).context(NoSuchManager{manager: manager.clone()})?;
            }

            if let Some(already) = manifest.data.contains_package(original.to_string()) {
                already.package_overloads.insert(manager, overload);
            } else {
                Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)).context(PackageNotAdded{package: original})?;
            }

        } else {
            added.package = package.to_string();
        }

        if let Some(packages) = &mut manifest.data.packages {
            packages.push(added);
        } else {
            manifest.data.packages = Some(vec![added]);
        }

        /* 
        if package.contains(":") {
            let mut added = AddedPackage::new();
            added.package = package.split(":").collect::<Vec<&str>>().get(0).unwrap().clone().to_string();
        } */
    }

    manifest.data.populate_file(&manifest);

    Ok(())
}
