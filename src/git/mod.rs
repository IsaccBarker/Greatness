pub mod pull;
pub mod push;
pub mod remote;
pub mod commit;
pub mod add;

use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum GitErrors {
    #[snafu(display("Failed to get index of git repository: {}", source))]
    FailedGitIndex {
        source: git2::Error,
    },

    #[snafu(display("Failed to get remote {} of git repository: {}", remote, source))]
    GitRemoteFindError {
        remote: String,
        source: git2::Error,
    }
}

