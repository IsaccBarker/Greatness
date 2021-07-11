use crate::manifest::State;
use crate::utils;
use clap::ArgMatches;
use snafu::{ResultExt, Snafu};
use std::path::PathBuf;

#[derive(Debug, Snafu)]
pub enum TagError {
    #[snafu(display(
        "File that is to be tagged greatly doesn't exist: {}. Please add it!",
        source
    ))]
    NoTrackedFileExistance { source: std::io::Error },
}

pub fn tag(matches: &ArgMatches, state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    for file in matches.values_of("files").unwrap() {
        tag_file(
            PathBuf::from(file),
            matches.value_of("tag").unwrap().to_string(),
            state,
        )?;
    }

    state.data.populate_file(state);

    Ok(())
}

pub fn tag_file(
    file: PathBuf,
    tag: String,
    state: &mut State,
) -> Result<(), Box<dyn std::error::Error>> {
    if !file.as_path().exists() {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "no such file or directory",
        ))
        .context(utils::NoFileExistsError { file: &file })?;
    }

    let normalized_file = utils::relative_to_special(&file)?;
    let mut contains = match state.data.contains(&normalized_file) {
        Some(c) => c.0.clone(),
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "no such file tracked",
            ))
            .context(NoTrackedFileExistance)?;
        }
    };

    contains.tag = Some(tag);

    state.data.add_file(contains);

    Ok(())
}
