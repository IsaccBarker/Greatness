use crate::manifest::State;
use clap::ArgMatches;
use log::{debug, info, warn};
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum PackageJogError {
    #[snafu(display("No supported package manager detected! Please check the docs for all supported package managers."))]
    NoManager { source: std::io::Error },

    #[snafu(display(
        "The package {} does not have an install name for {}. Please add one!",
        package,
        manager
    ))]
    PackageNoManager {
        package: String,
        manager: String,
        source: std::io::Error,
    },

    #[snafu(display(
        "The great package {} failed to install with manager {}: {}",
        package,
        manager,
        source
    ))]
    PackageInstallFail {
        package: String,
        manager: String,
        source: subprocess::PopenError,
    },
}

pub fn jog(_matches: &ArgMatches, manifest: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    let manager = match super::get_manager(manifest) {
        Some(m) => m,
        None => {
            Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)).context(NoManager {})?
        }
    };

    // TODO: Option to install all packages at once
    if let Some(packages) = &manifest.data.packages {
        for package in packages {
            let mut command = manifest.package_context.package_install_prefix.get_key_value(&manager).unwrap().0.clone();
            let package_name = package.package.clone();
            let mut args = manifest
                .package_context
                .package_install_prefix
                .get(&manager)
                .unwrap()
                .2
                .clone();

            args.push(package_name.clone());

            if package.package_overloads.len() != 0 {
                // We have overloads to deal with
                let mut to_use: (u8, String) = (0, "".into());
                /* for overload in &package.package_overloads {
                    let x = manifest.package_context.package_install_prefix.get(overload.0).unwrap();

                    if x.1 > to_use.0 {
                        to_use = (x.1, x.2.get(0).unwrap().clone());
                    }

                    println!("{:?}\n{:?}", &overload, &x);
                }

                println!("decided to move from {} to {}....", &command, &to_use.1);
                command = to_use.1; */
            }

            info!(
                "Installing package great {} with manager {}....",
                &package_name, &manager
            );
            debug!("{} {:?}", &command, &args);
            subprocess::Exec::cmd(command)
                .args(&args)
                .popen()
                .context(PackageInstallFail {
                    package: &package_name,
                    manager: &manager,
                })?;
        }
    } else {
        info!("No work to do!");
    }

    Ok(())
}
