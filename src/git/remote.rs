use crate::manifest::State;
use crate::utils;
use clap::ArgMatches;
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum RemoteErrors {
    #[snafu(display("Failed to set the remote of {} to {}: {}", name, url, source))]
    RemoteSetError {
        name: String,
        url: String,
        source: git2::Error,
    },
}

pub fn set_remote(
    matches: &ArgMatches,
    state: &mut State,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.value_of("remote").unwrap();
    let url = &utils::make_url_valid(matches.value_of("url").unwrap().to_string());

    if let Some(repo) = &mut state.repository {
        repo.remote(name, url)
            .context(RemoteSetError { name, url })?;
        repo.remote_set_url(name, url)
            .context(RemoteSetError { name, url })?;
    }

    Ok(())
}
