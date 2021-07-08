use crate::manifest::{AddedFile, State};
use crate::utils;
use clap::ArgMatches;
use log::debug;
use std::path::PathBuf;

/// Detects possible errors in the current configurations.
/// Currently checks for:
///     1. Simular tag names
///     2. Non-existant files
///     3. Non-existant scripts
pub fn doctor(manifest: &State, _matches: &ArgMatches) -> Option<Vec<String>> {
    let mut warnings = vec![];

    if let Some(files) = &manifest.data.files {
        debug!("Checking added files....");

        for file in files {
            debug!(
                "Checking added file {} and friends....",
                utils::special_to_absolute(&file.path).display()
            );

            // Tag checks
            {
                let tags = manifest.data.all_tags().unwrap_or(vec![]);
                check_tag_simularity(&mut warnings, &tags);
            }

            // Script checks
            {
                let scripts = manifest.data.all_scripts().unwrap_or(vec![]);
                check_script_existance(&mut warnings, &scripts);
            }

            // Dotfile checks
            {
                check_single_dotfile_existance(&mut warnings, &file);
            }
        }
    } else {
        debug!("No files to check!");
    }

    if let Some(_required) = &manifest.data.requires {
        debug!("Checking requirments....");
    } else {
        debug!("No requirements to check!");
    }

    if warnings.len() == 0 {
        return None;
    }
    Some(warnings)
}

fn check_tag_simularity(warnings: &mut Vec<String>, tags: &Vec<String>) {
    for tag_x in tags {
        for tag_y in tags {
            // Check if we are comparing the same tag
            if tag_x == tag_y {
                continue;
            }

            // Arbitrary, may require tuning.
            // Maybe turn this into an argument?
            if strsim::jaro(tag_x, tag_y) > 0.5 {
                warnings.push(format!(
                    "Great tags {} and {} are very similar. Did a great typo occur?",
                    tag_x, tag_y
                ));
            }
        }
    }
}

fn check_script_existance(warnings: &mut Vec<String>, scripts: &Vec<PathBuf>) {
    for script in scripts {
        if !script.exists() {
            warnings.push(format!(
                "Script {} doesn't exist!",
                utils::special_to_absolute(&script).display()
            ));
        }
    }
}

fn check_single_dotfile_existance(warnings: &mut Vec<String>, file: &AddedFile) {
    if !utils::special_to_absolute(&file.path).as_path().exists() {
        warnings.push(format!(
            "File {} doesn't exist!",
            utils::special_to_absolute(&file.path).display()
        ));
    }
}
