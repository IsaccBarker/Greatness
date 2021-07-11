use crate::manifest::State;
use clap::ArgMatches;
use log::{debug, info};
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

pub fn jog(_matches: &ArgMatches, state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    let manager = match super::get_manager(state) {
        Some(m) => m,
        None => {
            Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)).context(NoManager {})?
        }
    };

    // TODO: Option to install all packages at once
    if let Some(packages) = &state.data.packages {
        for package in packages {
            let mut command = state.package_context.package_install_prefix.get_key_value(&manager).unwrap().0.clone();
            let package_name = package.package.clone();
            let mut args = state
                .package_context
                .package_install_prefix
                .get(&manager)
                .unwrap()
                .2
                .clone();

            // Runs if we have overloads to deal with
            if package.package_overloads.len() != 0 {
                let mut to_use: (u8, String) = (0, "".into());
                for overload in &package.package_overloads {
                    let x = state.package_context.package_install_prefix.get_key_value(overload.0).unwrap();

                    if x.1.1 > to_use.0 {
                        to_use = (x.1.1, x.0.clone());
                    }
                }

                command = to_use.1;
            }


            // Runs if we need to run the command as root.
            if state.package_context.package_install_prefix.get(&command).unwrap().0 {
                args.insert(0, command);
                command = "sudo".into(); // TODO: Support doas.
            };

            args.push(package_name.clone());

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
