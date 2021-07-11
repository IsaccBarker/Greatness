use crate::manifest::State;
use crate::utils;
use log::info;

pub fn print_status(state: &State) {
    info!(
        "Greatness directory: \x1b[1m{}\x1b[0m",
        state.greatness_dir.display()
    );
    info!(
        "Greatness pulling  : \x1b[1m{}\x1b[0m",
        state.greatness_pulled_dir.display()
    );
    info!(
        "Greatness state : \x1b[1m{}\x1b[0m",
        state.greatness_state.display()
    );
    info!(
        "Greatness git pack : \x1b[1m{}\x1b[0m",
        state.greatness_git_pack_dir.display(),
    );
    info!(
        "Greatness scripts  : \x1b[1m{}\x1b[0m",
        state.greatness_scripts_dir.display()
    );

    print!("\n");

    if let Some(packages) = &state.data.packages {
        info!("Packages: ");

        for package in packages {
            info!(
                "\tname: {}",
                package.package
            );

            if package.package_overloads.len() != 0 {
                info!("\t\toverlods:");
                for (manager, overload) in package.package_overloads.clone().into_iter() {
                    info!("\t\t\tmanager: {}", manager);
                    info!("\t\t\toverload: {}", overload);
                }
            }
        }
    }

    if let Some(files) = &state.data.files {
        info!("Added files:");

        for file in files {
            info!(
                "\tpath: {}",
                utils::special_to_absolute(&file.path).display()
            );

            if file.tag.is_some() && file.tag != Some("".to_owned()) {
                info!("\t\ttag: {}", file.tag.clone().unwrap());
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

    if let Some(requires) = &state.data.requires {
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
