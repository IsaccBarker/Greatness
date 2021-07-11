use crate::git::clone;
use crate::init;
use crate::manifest::{Manifest, State};
use crate::package;
use crate::script;
use crate::utils;
use clap::ArgMatches;
use log::{debug, info, warn};
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
    state: &mut State,
    sub_state: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if matches.is_present("as-main") {
        debug!("Installing as main!");
    }

    // Normallize the URL and get a valid location to clone to
    let (url, mut clone_to) = get_git_pair(state, user_url, matches);

    // Create the clone to directory if none exists
    if clone_to.exists() {
        std::fs::remove_dir_all(&clone_to).context(RemoveFailure { dir: &clone_to })?;
    }

    // TODO: Implement a progress bar. https://docs.rs/git2/0.13.20/git2/struct.Progress.html
    info!("Cloning from {} into {}....", url, &clone_to.display());
    clone::clone_repo(&url, &clone_to)?;

    // Parse the file. False as we want to enable git
    let mut external_state = State::new(PathBuf::from(clone_to.to_str().unwrap()))?;
    external_state.data = Manifest::populate_from_file(&&external_state)?;

    install(
        matches,
        Some(url),
        &mut clone_to,
        state,
        &mut external_state,
        sub_state,
    )?;
    for requirement in external_state.data.requires.unwrap_or(vec![]) {
        clone_and_install_repo(
            requirement.0.unwrap_or("".to_string()),
            matches,
            state,
            true,
        )?;
    }

    if matches.is_present("as-main") {
        init::init_no_damage(matches, state)?;
    }

    Ok(())
}

/// Return a tuple contains the URL of a repository and where
/// to clone it to.
fn get_git_pair(state: &State, user_url: String, matches: &ArgMatches) -> (String, PathBuf) {
    let url = utils::make_url_valid(user_url);

    let mut clone_to = PathBuf::from(&state.greatness_pulled_dir);
    #[allow(unused_assignments)]
    let mut dest_tmp = url.replace("https://", "");
    dest_tmp = url.replace("http://", "");
    dest_tmp = dest_tmp.replace(".git", "");

    if !matches.is_present("as-main") {
        let dest: PathBuf = PathBuf::from(
            dest_tmp
                .split("/")
                .collect::<Vec<&str>>()
                .join(std::path::MAIN_SEPARATOR.to_string().as_str()),
        );

        clone_to.push(&dest);
    } else {
        clone_to = state.greatness_dir.clone();
    }

    (url, clone_to)
}

/// Install external from a local directory
/// * `url` - Optional URL that it was cloned from. Is used to update when wanted.
/// * `from` - Where the external state is located on disk.
/// * `manfiest` - State to write into.
pub fn install(
    matches: &ArgMatches,
    url: Option<String>,
    install_from: &mut PathBuf,
    state: &mut State,
    external_state: &mut State,
    sub_state: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let external_state_data = external_state.data.clone();
    if let Some(files) = external_state_data.files {
        install_from.push("files");

        for file in files {
            if matches.is_present("only-with-tag")
                && matches.value_of("only-with-tag").unwrap() == file.tag.unwrap_or("".to_owned())
            {
                continue;
            }

            install_file(install_from, file.path)?;
        }
    }

    install_mods(matches, external_state)?;

    // Make sure we mark this as a dependency, only if we are not
    // installing it as main
    if !matches.is_present("as-main") {
        mark_as_dependency(state, install_from, url, sub_state);
    } else {
        debug!("--as-main specified, not marking specfied as a dependency....");
    }

    // Don't overwrite the state with nothing if
    // we plan to pull as main.
    if !matches.is_present("as-main") {
        state.data.populate_file(state);
    }

    Ok(())
}

pub fn install_mods(
    matches: &ArgMatches,
    external_state: &mut State,
) -> Result<(), Box<dyn std::error::Error>> {
    // Run the scripts, and install the packages.
    if matches.is_present("allow-mods") {
        debug!("--allow-mods specified, running scripts....");
        script::jog::jog(external_state)?;

        debug!("--allow-mods specified, installing packages....");
        package::jog::jog(matches, external_state)?;
    } else {
        warn!("The --allow-mods (-d) argument is not passed! No scripts will be run for security reasons :D");
    }

    Ok(())
}

fn mark_as_dependency(
    state: &mut State,
    install_from: &mut PathBuf,
    url: Option<String>,
    sub_state: bool,
) {
    if let Some(requires) = &mut state.data.requires {
        let mut add = true;

        // Check is already added. We do this last because the user may want to re-merge everything
        requires.iter().for_each(|x| {
            if x.1 == utils::absolute_to_special(install_from) {
                add = false;
            }
        });

        if add && !sub_state {
            requires.push((url, utils::absolute_to_special(&install_from.clone())));
        }
    } else {
        state.data.requires = Some(vec![(
            url,
            utils::absolute_to_special(&install_from.clone()),
        )]);
    }
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

        info!("{} already exists (which is great)!", install_to.display());
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
