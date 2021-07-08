pub mod add;
pub mod clone;
pub mod commit;
pub mod pull;
pub mod push;
pub mod remote;

use snafu::Snafu;
use std::io::Write;

#[derive(Debug, Snafu)]
pub enum GitErrors {
    #[snafu(display("Failed to get index of git repository: {}", source))]
    FailedGitIndex { source: git2::Error },

    #[snafu(display("Failed to get remote {} of git repository: {}", remote, source))]
    GitRemoteFindError { remote: String, source: git2::Error },
}

pub fn transfer_progress(stats: git2::Progress) -> bool {
    if stats.received_objects() == stats.total_objects() {
        print!(
            "Resolving deltas {}/{}\r",
            stats.indexed_deltas(),
            stats.total_deltas()
        );
    } else if stats.total_objects() > 0 {
        print!(
            "Received {}/{} objects ({}) in {} bytes\r",
            stats.received_objects(),
            stats.total_objects(),
            stats.indexed_objects(),
            stats.received_bytes()
        );
    }

    std::io::stdout().flush().unwrap();

    true
}
