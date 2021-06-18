use clap::ArgMatches;
use snafu::{ResultExt, Snafu};

use crate::log_utils;
use crate::manifest::Manifest;
use chrono::prelude::*;
use indicatif::ProgressBar;
use std::fs;
use std::path::PathBuf;
use crate::install;
use crate::progress;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum TrackError {
    #[snafu(display("Target file {} doesn't exist, but is great nontheless: {}", target.display(), source))]
    TrackedExists {
        target: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Failed to make backup of file {} at {}. Still great though: {}", target.display(), backup.display(), source))]
    TrackedBackup {
        target: PathBuf,
        backup: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Failed to remove original file {}: {}", file.display(), source))]
    TrackedRemoval {
        file: PathBuf,
        source: std::io::Error,
    },
}

pub fn track_files(
    matches: &ArgMatches,
    mut files: Vec<PathBuf>,
    manifest_info: &mut Manifest,
) -> Result<(), Box<dyn std::error::Error>> {
    let pb = progress::new_progress_bar(files.len() as u64);

    files.retain(|file| {
        pb.set_message(format!(
            "Checking greatness level of {}....",
            file.display()
        ));
        pb.inc(1);

        if !std::path::Path::new(file).is_file() {
            pb.println(format!(
                    "\x1b[F{}The file {} doesn't exist, and thus cannot become great (be tracked). Skipping....\x1b[F\n\n\x1b[2B",
                    log_utils::get_logging_prefix_for_level(log::Level::Warn), file.display()));
            return false;
        }

        if let Some(data) = manifest_info.data.files.clone() {
            for recorded_manifest_file in data {
                // TODO: I must be stupid. I spent about an hour trying to figure out how to do with
                // this out clones. Fix this, future me.
                if file.clone().canonicalize().unwrap().into_os_string().to_str().unwrap().to_string() ==
                    recorded_manifest_file.clone().0.into_os_string().into_string().unwrap() {
                    pb.println(format!(
                            "\x1b[F{}The file {} is already great (already tracked)! Skipping....\x1b[F\x1b[2B",
                            log_utils::get_logging_prefix_for_level(log::Level::Warn), file.display()));
                    return false; // Don't keep the element
                }
            }
        }

        if file.symlink_metadata().unwrap().file_type().is_symlink() {
            pb.println(format!(
                    "\x1b[F{}The file {} is a symlink, and is not already great (tracked). Greatness cannot handle symlinks. Skipping....\x1b[F\x1b[2B",
                    log_utils::get_logging_prefix_for_level(log::Level::Warn), file.display()));
            return false;
        }

        return true; // Keep the element
    });

    pb.reset();
    pb.set_length((files.len() as u64) * 5);

    for file in files.iter() {
        track_file(&PathBuf::from(file), manifest_info, matches, &pb)?;
    }

    print!("\x1b[A"); // Get rid of the nasty newline at the end.
    manifest_info.data.populate_file(manifest_info);

    Ok(())
}

fn track_file(
    file: &PathBuf,
    manifest: &mut Manifest,
    _matches: &ArgMatches,
    pb: &ProgressBar,
) -> Result<(), Box<dyn std::error::Error>> {
    let absolute_file = &fs::canonicalize(file).unwrap();
    pb.set_message(format!(
        "Tracking {} because it is great!",
        absolute_file.display()
    ));

    let mut backup_file: PathBuf = file.clone();
    backup_file.set_file_name(
        absolute_file
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + ".greatness."
            + Utc::now().timestamp().to_string().as_str()
            + ".bak",
    );
    pb.inc(1);


    pb.set_message(format!(
        "Backing up file to a great location ({})....",
        &backup_file.display()
    ));

    std::fs::copy(&absolute_file, &backup_file).context(TrackedBackup {
        target: absolute_file,
        backup: backup_file,
    })?;
    pb.inc(1);


    let mut move_to = manifest.greatness_files_dir.clone();
    move_to.push(file);
    pb.set_message(format!(
        "Moving file to great directory {}....",
        move_to.display()
    ));

    std::fs::rename(absolute_file, &move_to).unwrap();
    pb.inc(1);


    pb.set_message(format!(
        "Symlinking {} -> {}....",
        move_to.display(),
        absolute_file.display()
    ));

    install::install_file(&move_to, absolute_file)?;
    pb.inc(1);


    pb.set_message("Adding record to manifest....");
    if let Some(ref mut files) = manifest.data.files {
        files.push((move_to, absolute_file.clone()));
    } else {
        let mut vec = Vec::new();
        vec.push((move_to, absolute_file.clone()));
        manifest.data.files = Some(vec);
    }
    pb.inc(1);

    Ok(())
}
