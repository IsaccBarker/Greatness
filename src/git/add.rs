use crate::manifest::State;
use clap::ArgMatches;
use snafu::{Snafu, ResultExt};

#[derive(Debug, Snafu)]
pub enum AddErrors {
    #[snafu(display("Failed to add files: {}", source))]
    AddFilesError {
        source: git2::Error,
    },
}

pub fn add(_matches: &ArgMatches, manifest: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(repo) = &manifest.repository {
        let mut index = repo.index().context(super::FailedGitIndex{})?;

        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None).context(AddFilesError{})?;

        index.write()?;
    }

    Ok(())
}

