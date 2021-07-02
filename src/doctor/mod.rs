use crate::manifest::Manifest;
use crate::utils;
use clap::ArgMatches;
use log::debug;

/// Detects possible errors in the current configurations.
/// Currently checks for:
///     1. Simular tag names
///     2. Non-existant files
///
pub fn doctor(manifest: &Manifest, _matches: &ArgMatches) -> Option<Vec<String>> {
    let mut warnings = vec![];

    if let Some(files) = &manifest.data.files {
        debug!("Checking added files....");

        for file in files {
            debug!(
                "Checking added file {}....",
                utils::special_to_absolute(&file.path).display()
            );

            {
                let tags = manifest.data.all_tags().unwrap_or(vec![]);
                for tag_x in &tags {
                    for tag_y in &tags {
                        // Check if we are comparing the same tag
                        if tag_x == tag_y {
                            continue;
                        }

                        // Arbitrary, may require tuning.
                        // Maybe turn this into an argument?
                        if strsim::jaro(tag_x, tag_y) > 0.5 {
                            warnings.push(format!(
                                "Great tags {} and {} are very similar. Did a great type occur?",
                                tag_x, tag_y
                            ));
                        }
                    }
                }
            }

            {
                if !utils::special_to_absolute(&file.path).as_path().exists() {
                    warnings.push(format!(
                        "File {} doesn't exist!",
                        utils::special_to_absolute(&file.path).display()
                    ));
                }
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
