use crate::manifest::Manifest;
use snafu::{ResultExt, Snafu};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

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

/// Transforms an absolute path to a special one.
/// /home/milo/.zshrc -> {{HOME}}/.zshrc
pub fn absolute_to_special(absolute: &PathBuf) -> PathBuf {
    let home_to_replace = home::home_dir().unwrap();
    PathBuf::from(
        absolute
            .to_str()
            .unwrap()
            .replace(home_to_replace.to_str().unwrap(), "{{HOME}}"),
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
pub fn relative_to_script(manifest: &Manifest, rel: &PathBuf) -> PathBuf {
    let mut ret = manifest.greatness_scripts_dir.clone();
    ret.push(rel);

    ret
}

/// Given a origin file, create a backup file with a unique name
pub fn backup_file(original: &PathBuf) -> Result<(), CommonErrors> {
    let mut backup = original.clone();
    // https://softwareengineering.stackexchange.com/questions/339125/acceptable-to-rely-on-random-ints-being-unique

    let datetime = SystemTime::now().duration_since(UNIX_EPOCH).expect("woah dude, something got fucked up. time went backwards. what the hell is going on. HELP!").as_secs();
    backup.set_extension(
        backup
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap()
            .to_string()
            + datetime.to_string().as_str()
            + ".bak",
    );

    std::fs::copy(&original, &backup).context(FileCopyError {
        src: &original,
        dest: backup,
    })?;

    Ok(())
}
