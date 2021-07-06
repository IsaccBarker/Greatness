use git2::Repository;
use snafu::{Snafu, ResultExt};
use std::path::PathBuf;
use std::io::Write;

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

struct State {
    progress: Option<git2::Progress<'static>>,
    total: usize,
    current: usize,
    path: Option<PathBuf>,
    newline: bool,
}

/// Clones a repository from url to clone_to.
pub fn clone_repo(url: &String, clone_to: &PathBuf) -> Result<(), CloneError> {
    let state = std::cell::RefCell::new(State {
        progress: None,
        total: 0,
        current: 0,
        path: None,
        newline: false,
    });

    let mut cb = git2::RemoteCallbacks::new();
    cb.transfer_progress(|stats| {
        let mut state = state.borrow_mut();
        state.progress = Some(stats.to_owned());
        clone_progress(&mut *state);
        true
    });

    let mut co = git2::build::CheckoutBuilder::new();
    co.progress(|path, cur, total| {
        let mut state = state.borrow_mut();
        state.path = path.map(|p| p.to_path_buf());
        state.current = cur;
        state.total = total;
        clone_progress(&mut *state);
    });

    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(cb);
    git2::build::RepoBuilder::new()
        .fetch_options(fo)
        .with_checkout(co)
        .clone(&url, &clone_to).context(CloneFailure {
            url,
            dest: clone_to,
        })?;
    println!();

    Ok(())
}

fn clone_progress(state: &mut State) {
    let stats = state.progress.as_ref().unwrap();
    let network_pct = (100 * stats.received_objects()) / stats.total_objects();
    let index_pct = (100 * stats.indexed_objects()) / stats.total_objects();
    let co_pct = if state.total > 0 {
        (100 * state.current) / state.total
    } else {
        0
    };
    let kbytes = stats.received_bytes() / 1024;
    if stats.received_objects() == stats.total_objects() {
        if !state.newline {
            println!();
            state.newline = true;
        }
        print!(
            "Resolving deltas {}/{}\r",
            stats.indexed_deltas(),
            stats.total_deltas()
        );
    } else {
        print!(
            "net {:3}% ({:4} kb, {:5}/{:5})  /  idx {:3}% ({:5}/{:5})  \
             /  chk {:3}% ({:4}/{:4}) {}\r",
            network_pct,
            kbytes,
            stats.received_objects(),
            stats.total_objects(),
            index_pct,
            stats.indexed_objects(),
            stats.total_objects(),
            co_pct,
            state.current,
            state.total,
            state
                .path
                .as_ref()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default()
        )
    }

    std::io::stdout().flush().unwrap();
}

