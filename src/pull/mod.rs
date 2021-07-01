use crate::manifest::{Manifest, ManifestData};
use crate::utils;
use clap::ArgMatches;
use git2::Repository;
use log::{debug, info};
use question::{Answer, Question};
use snafu::{ResultExt, Snafu};
use std::path::PathBuf;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
/// Errors pretaining to the cloning of repositories
pub enum CloneError {
    #[snafu(display("Failed to clone great repository {} into great location {}: {}", url, dest.display(), source))]
    CloneFailure {
        url: String,
        dest: PathBuf,
        source: git2::Error,
    },

    #[snafu(display("Failed to remove pre-pulling directory {}: {}", dir.display(), source))]
    RemoveFailure {
        dir: PathBuf,
        source: std::io::Error,
    },
}

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
/// Errors pretaining to the installation of repositories
pub enum InstallError {
    #[snafu(display("Failed to backup file ({} of {}) that already exists: {}", dest.display(), src.display(), source))]
    BackupFile {
        src: PathBuf,
        dest: PathBuf,
        source: std::io::Error,
    },
}

/// Clone and install a repository
pub fn clone_and_install_repo(
    user_url: String,
    matches: &ArgMatches,
    manifest: &mut Manifest,
    sub_manifest: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if matches.is_present("as-main") {
        debug!("Installing as main!");
    }

    // Normallize the URL and get a valid location to clone to
    let (url, mut clone_to) = get_git_pair(manifest, user_url, matches);

    // Create the clone to directory if none exists
    if clone_to.exists() {
        std::fs::remove_dir_all(&clone_to).context(RemoveFailure { dir: &clone_to })?;
    }
    
    // TODO: Implement a progress bar. https://docs.rs/git2/0.13.20/git2/struct.Progress.html
    info!("Cloning from {} into {}....", url, &clone_to.display());
    clone_repo(&url, &clone_to)?;

    // Parse the file. False as we want to enable git
    let mut external_manifest = Manifest::new(PathBuf::from(clone_to.to_str().unwrap()))?;
    external_manifest.data = ManifestData::populate_from_file(&&external_manifest)?;

    install(
        matches,
        Some(url),
        &mut clone_to,
        manifest,
        &external_manifest,
        sub_manifest,
    )?;
    for requirement in external_manifest.data.requires.unwrap_or(vec![]) {
        clone_and_install_repo(
            requirement.0.unwrap_or("".to_string()),
            matches,
            manifest,
            true,
        )?;
    }

    Ok(())
}

/// Return a tuple contains the URL of a repository and where
/// to clone it to.
fn get_git_pair(manifest: &Manifest, user_url: String, matches: &ArgMatches) -> (String, PathBuf) {
    let url = make_url_valid(user_url);

    let mut clone_to = PathBuf::from(&manifest.greatness_pulled_dir);
    #[allow(unused_assignments)]
    let mut dest_tmp = url.replace("https://", "");
    dest_tmp = url.replace("http://", "");
    dest_tmp = dest_tmp.replace(".git", "");

    if ! matches.is_present("as-main") {
        let dest: PathBuf = PathBuf::from(
            dest_tmp
                .split("/")
                .collect::<Vec<&str>>()
                .join(std::path::MAIN_SEPARATOR.to_string().as_str()),
        );

        clone_to.push(&dest);
    } else {
        clone_to = manifest.greatness_dir.clone();
    }

    (url, clone_to)
}

/// Clones a repository from url to clone_to.
fn clone_repo(url: &String, clone_to: &PathBuf) -> Result<(), CloneError> {
    Repository::clone(&url, &clone_to).context(CloneFailure {
        url,
        dest: clone_to,
    })?;

    Ok(())
}

/// Transmute urls into something git can handle. For example:
/// Pattern 	        HTTPS Repo
/// user 	            https://github.com/user/dotfiles.git
/// user/repo 	        https://github.com/user/repo.git
fn make_url_valid(url: String) -> String {
    let mut new: Vec<&str> = Vec::new();

    if !url.contains("https://") {
        new.push("https://");
    }

    if url.matches("/").count() == 1 {
        // Assume its github
        new.push("github.com/");
    }

    new.push(&url);

    if !url.contains(".git") {
        new.push(".git");
    }

    new.join("")
}

/// Install external from a local directory
/// * `url` - Optional URL that it was cloned from. Is used to update when wanted.
/// * `from` - Where the external manifest is located on disk.
/// * `manfiest` - Manifest to write into.
pub fn install(
    matches: &ArgMatches,
    url: Option<String>,
    install_from: &mut PathBuf,
    manifest: &mut Manifest,
    external_manifest: &Manifest,
    sub_manifest: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let external_manifest_data = external_manifest.data.clone();
    if let Some(files) = external_manifest_data.files {
        install_from.push("files");

        for file in files {
            if matches.is_present("only-with-tag") && matches.value_of("only-with-tag").unwrap() == file.tag.unwrap_or("".to_owned()) {
                continue;
            }

            install_file(install_from, file.path)?;
        }
    }

    // Merge it all

    // Make sure we mark this as a dependency, only if we are not
    // installing it as main
    if ! matches.is_present("as-main") {
        if let Some(requires) = &mut manifest.data.requires {
            let mut add = true;

            // Check is already added. We do this last because the user may want to re-merge everything
            requires.iter().for_each(|x| {
                if x.1 == utils::absolute_to_special(install_from) {
                    add = false;
                }
            });

            if add && !sub_manifest {
                requires.push((url, utils::absolute_to_special(&install_from.clone())));
            }
        } else {
            manifest.data.requires = Some(vec![(
                url,
                utils::absolute_to_special(&install_from.clone()),
            )]);
        }
    } else {
        debug!("--as-main specified, not marking specfied as a dependency....");
    }

    // Don't overwrite the manifest with nothing if
    // we plan to pull as main.
    if ! matches.is_present("as-main") {
        manifest.data.populate_file(manifest);
    }

    Ok(())
}

/// Install a file
fn install_file(install_from: &PathBuf, file: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut install_from_now = install_from.clone();
    let mut install_from_intr: PathBuf = PathBuf::from(std::path::MAIN_SEPARATOR.to_string()); // Using "/" here simply makes home/milo turn into /home/milo, so we can replace it with {{HOME}}
    let install_to = utils::special_to_absolute(&file);

    install_to
        .to_str()
        .unwrap()
        .to_string()
        .split(std::path::MAIN_SEPARATOR)
        .into_iter()
        .for_each(|e| install_from_intr.push(e));
    install_from_now.push(utils::absolute_to_special(&install_from_intr));

    debug!(
        "Installing great file to great location; {} to {}....",
        install_from_now.display(),
        install_to.display()
    );

    if install_to.as_path().exists() {
        // We need to make a backup

        info!("{} already exists (which is great!) Specify the -y (for yes) or the -n (for no) options to skip user input.", install_to.display());
        info!("Note that skipping doing this could cause the dotfiles you are pulling and merging to not work. A backup WILL be made!");
        let answer = Question::new("Do you want to overwrite?")
            .default(Answer::YES)
            .show_defaults()
            .confirm();

        if answer != Answer::YES {
            info!("Skipping....");
            return Ok(());
        }

        utils::backup_file(&install_to)?;
    } else {
        // Create the directories we need to house the file
        // that is to be installed
        create_dirs_for_file_install(&install_to)?;
    }

    std::fs::copy(&install_from_now, &install_to).context(utils::FileCopyError {
        src: &install_from_now,
        dest: &install_to,
    })?;

    Ok(())
}

fn create_dirs_for_file_install(install_to: &PathBuf) -> Result<(), utils::CommonErrors> {
    let as_vec = &install_to.to_str().unwrap().to_string();
    let splitted = as_vec.split(std::path::MAIN_SEPARATOR);
    let mut dirs_to_create = splitted.clone().collect::<Vec<&str>>();
    dirs_to_create.remove(dirs_to_create.len() - 1);
    let str_dirs_to_create = dirs_to_create.join(std::path::MAIN_SEPARATOR.to_string().as_str());

    std::fs::create_dir_all(&str_dirs_to_create).context(utils::DirCreationError {
        dir: PathBuf::from(str_dirs_to_create),
    })?;
    std::fs::File::create(&install_to).context(utils::FileCreationError { file: &install_to })?;

    Ok(())
}
