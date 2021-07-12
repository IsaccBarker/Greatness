use crate::manifest::State;
use snafu::{ResultExt, Snafu};
use std::path::PathBuf;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum CommonErrors {
    #[snafu(display("Not-yet-great file {} is not yet tracked!", file.display()))]
    FileNotTracked {
        file: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Great file {} does not exist!", file.display()))]
    NoFileExistsError {
        file: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Failed to open great file {}: {}", file.display(), source))]
    FileOpenError {
        file: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Failed to create great file {}: {}", file.display(), source))]
    FileCreationError {
        file: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Failed to delete great file {}: {}", file.display(), source))]
    FileDeletionError {
        file: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Failed to read great file {}: {}", file.display(), source))]
    FileReadError {
        file: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Failed to write great file {}: {}", file.display(), source))]
    FileWriteError {
        file: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Failed to create great directory {}: {}", dir.display(), source))]
    DirCreationError {
        dir: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Failed to delete great directory {}: {}", dir.display(), source))]
    DirDeletionError {
        dir: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Failed to copy great file from {} -> {}: {}", src.display(), dest.display(), source))]
    FileCopyError {
        src: PathBuf,
        dest: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Failed to change directories to {}: {}", dir.display(), source))]
    ChangeDirError {
        dir: PathBuf,
        source: std::io::Error,
    },
}

/// Transmute urls into something git can handle. For example:
/// Pattern 	        HTTPS Repo
/// user 	            https://github.com/user/dotfiles.git
/// user/repo 	        https://github.com/user/repo.git
pub fn make_url_valid(url: String) -> String {
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

/// Transforms an absolute path to a special one.
/// /home/milo/.zshrc -> {{HOME}}/.zshrc
pub fn absolute_to_special(absolute: &PathBuf) -> PathBuf {
    // Abstract this data into a singleton so we only have to popualte it once?
    let home_to_replace = home::home_dir().unwrap();
    let mut desktop_to_replace = home_to_replace.clone();
    desktop_to_replace.push("/Desktops");
    let mut documents_to_replace = home_to_replace.clone();
    documents_to_replace.push("/Documents");
    let mut downloads_to_replace = home_to_replace.clone();
    downloads_to_replace.push("/Downloads");
    let mut music_to_replace = home_to_replace.clone();
    music_to_replace.push("/Music");
    let mut pictures_to_replace = home_to_replace.clone();
    pictures_to_replace.push("/Pictures");
    let mut publicshare_to_replace = home_to_replace.clone();
    publicshare_to_replace.push("/Public");
    let mut templates_to_replace = home_to_replace.clone();
    templates_to_replace.push("/Templates");
    let mut video_to_replace = home_to_replace.clone();
    video_to_replace.push("/Videos");

    if std::env::var("XDG_PUBLICSHARE_DIR").unwrap_or("".to_owned()) != "" {
        // XDG is installed
        desktop_to_replace = PathBuf::from(std::env::var("XDG_DESKTOP_DIR").unwrap());
        documents_to_replace = PathBuf::from(std::env::var("XDG_DOCUMENTS_DIR").unwrap());
        downloads_to_replace = PathBuf::from(std::env::var("XDG_DOWNLOAD_DIR").unwrap());
        music_to_replace = PathBuf::from(std::env::var("XDG_MUSIC_DIR").unwrap());
        pictures_to_replace = PathBuf::from(std::env::var("XDG_PICTURES_DIR").unwrap());
        publicshare_to_replace = PathBuf::from(std::env::var("XDG_PUBLICSHARE_DIR").unwrap());
        templates_to_replace = PathBuf::from(std::env::var("XDG_TEMPLATES_DIR").unwrap());
        video_to_replace = PathBuf::from(std::env::var("XDG_VIDEOS_DIR").unwrap());
    }

    PathBuf::from(
        absolute
            .to_str()
            .unwrap()
            .replace(home_to_replace.to_str().unwrap(), "{{HOME}}")
            .replace(desktop_to_replace.to_str().unwrap(), "{{DESKTOP}}")
            .replace(downloads_to_replace.to_str().unwrap(), "{{DOWNLOADS}}")
            .replace(documents_to_replace.to_str().unwrap(), "{{DOCUMENTS}}")
            .replace(music_to_replace.to_str().unwrap(), "{{MUSIC}}")
            .replace(pictures_to_replace.to_str().unwrap(), "{{PICTURES}}")
            .replace(publicshare_to_replace.to_str().unwrap(), "{{PUBLICSHARE}}")
            .replace(templates_to_replace.to_str().unwrap(), "{{TEMPLATES}}")
            .replace(video_to_replace.to_str().unwrap(), "{{VIDEOS}}")
    )
}

/// Calls absolute_to_special, but calls .cannonicalize()
/// on the relative path first.
pub fn relative_to_special(relative: &PathBuf) -> Result<PathBuf, std::io::Error> {
    Ok(absolute_to_special(&relative.canonicalize()?))
}

/// Transforms a special path to an absolute one.
/// {{HOME}}/.zshrc -> /home/milo/.zshrc
pub fn special_to_absolute(special: &PathBuf) -> PathBuf {
    let home_to_substitute = home::home_dir().unwrap();
    let special_string = special.to_str().unwrap().to_string();

    PathBuf::from(special_string.replace("{{HOME}}", home_to_substitute.to_str().unwrap()))
}

/// Supplied a relative path, this function returns that
/// scripts location in the script directory
pub fn relative_to_script(state: &State, rel: &PathBuf) -> PathBuf {
    let mut ret = state.greatness_scripts_dir.clone();
    ret.push(rel);

    ret
}

/// Given a origin file, create a backup file with a unique name
pub fn backup_file(original: &PathBuf) -> Result<(), CommonErrors> {
    let mut backup = original.clone();
    backup.set_extension(
        backup
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap()
            .to_string()
            + ".bak",
    );

    std::fs::copy(&original, &backup).context(FileCopyError {
        src: &original,
        dest: backup,
    })?;

    Ok(())
}
