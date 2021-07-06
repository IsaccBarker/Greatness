use crate::manifest::Manifest;
use clap::ArgMatches;
use snafu::{Snafu, ResultExt};
use log::warn;

#[derive(Debug, Snafu)]
pub enum PushErrors {
    #[snafu(display("Failed to add files: {}", source))]
    AddFilesError {
        source: git2::Error,
    },

    #[snafu(display("Failed to push files: {}", source))]
    PushFileError {
        source: git2::Error,
    }
}

pub fn push(matches: &ArgMatches, manifest: &mut Manifest) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(repo) = &manifest.repository {
        let mut remote = repo.find_remote(matches.value_of("remote").unwrap()).context(super::GitRemoteFindError{remote: "remote".to_owned()})?;
        let mut opts = git2::PushOptions::new();
        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(|url, _username_from_url, _allowed_types| {
            println!("{:?}", _allowed_types);

            warn!("Username and password authentication required for url {}! Note that if you have a GitHub PAT, use that instead of the password :D", url);
            let username = match question::Question::new("Username: ").ask().unwrap() {
                question::Answer::RESPONSE(r) => r,
                _ => unreachable!(),
            };

            let password = match question::Question::new("Password: ").ask().unwrap() {
                question::Answer::RESPONSE(r) => r,
                _ => unreachable!(),
            };

            git2::Cred::userpass_plaintext(&username, &password)
        });

        opts.remote_callbacks(callbacks);
        
        remote.push(vec!["+refs/heads/".to_string() + matches.value_of("branch").unwrap()].as_slice(), Some(&mut opts)).context(PushFileError{})?;
    }

    Ok(())
}
