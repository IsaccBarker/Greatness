use std::path::PathBuf;

/// Transforms an absolute path to a special one.
/// /home/milo/.zshrc -> {{HOME}}/.zshrc
pub fn absolute_to_special(absolute: &PathBuf) -> PathBuf {
    let home_to_replace = home::home_dir().unwrap();
    PathBuf::from(absolute.to_str().unwrap().replace(home_to_replace.to_str().unwrap(), "{{HOME}}"))
}

/// Transforms a special path to an absolute one.
/// {{HOME}}/.zshrc -> /home/milo/.zshrc
pub fn special_to_absolute(special: &PathBuf) -> PathBuf {
    let home_to_substitute = home::home_dir().unwrap();
    let special_string = special.to_str().unwrap().to_string();

    PathBuf::from(special_string.replace("{{HOME}}", home_to_substitute.to_str().unwrap()))
}

