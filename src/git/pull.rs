use crate::manifest::State;
use clap::ArgMatches;
use log::{info, warn};
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum PullErrors {
    #[snafu(display("Branch {} doesn't exist: {}", branch, source))]
    BranchDoesntExist { branch: String, source: git2::Error },
}

#[allow(dead_code)]
fn fast_forward(
    repo: &git2::Repository,
    lb: &mut git2::Reference,
    rc: &git2::AnnotatedCommit,
) -> Result<(), git2::Error> {
    let name = match lb.name() {
        Some(s) => s.to_string(),
        None => String::from_utf8_lossy(lb.name_bytes()).to_string(),
    };

    let msg = format!("Fast-Forward: Setting {} to id: {}", name, rc.id());
    info!("Fast-Forward: Setting {} to id: {}", name, rc.id());

    lb.set_target(rc.id(), &msg)?;
    repo.set_head(&name)?;
    repo.checkout_head(Some(
        git2::build::CheckoutBuilder::default()
            // For some reason the force is required to make the working directory actually get updated
            // I suspect we should be adding some logic to handle dirty working directory States.
            .force(),
    ))?;
    Ok(())
}

#[allow(dead_code)]
fn normal_merge(
    repo: &git2::Repository,
    local: &git2::AnnotatedCommit,
    remote: &git2::AnnotatedCommit,
) -> Result<(), git2::Error> {
    let local_tree = repo.find_commit(local.id())?.tree()?;
    let remote_tree = repo.find_commit(remote.id())?.tree()?;
    let ancestor = repo
        .find_commit(repo.merge_base(local.id(), remote.id())?)?
        .tree()?;
    let mut idx = repo.merge_trees(&ancestor, &local_tree, &remote_tree, None)?;

    if idx.has_conflicts() {
        warn!("Merge conficts detected...");
        repo.checkout_index(Some(&mut idx), None)?;
        return Ok(());
    }

    let result_tree = repo.find_tree(idx.write_tree_to(repo)?)?;

    // now create the merge commit
    let msg = format!("Merge: {} into {}", remote.id(), local.id());
    let sig = repo.signature()?;
    let local_commit = repo.find_commit(local.id())?;
    let remote_commit = repo.find_commit(remote.id())?;

    // Do our merge commit and set current branch head to that commit.
    let _merge_commit = repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &msg,
        &result_tree,
        &[&local_commit, &remote_commit],
    )?;

    // Set working tree to match head.
    repo.checkout_head(None)?;
    Ok(())
}

pub fn pull(matches: &ArgMatches, manifest: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    let mut cb = git2::RemoteCallbacks::new();
    cb.transfer_progress(super::transfer_progress);

    let mut fo = git2::FetchOptions::new();
    fo.download_tags(git2::AutotagOption::All);

    if let Some(repo) = &manifest.repository {
        let mut remote = repo
            .find_remote(matches.value_of("remote").unwrap())
            .context(super::GitRemoteFindError {
                remote: "remote".to_owned(),
            })?;
        let remote_branch = &[matches.value_of("branch").unwrap()];

        remote.fetch(remote_branch, Some(&mut fo), None)?;
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;

        // Do a merge analysis
        let analysis = repo.merge_analysis(&[&fetch_commit])?;

        if analysis.0.is_fast_forward() {
            info!("Doing a fast forward....");
            // do a fast forward
            let refname = "refs/heads/".to_string() + remote_branch.get(0).unwrap();
            let mut r = repo.find_reference(&refname).context(BranchDoesntExist {
                branch: matches.value_of("branch").unwrap(),
            })?;
            fast_forward(repo, &mut r, &fetch_commit)?;
        } else if analysis.0.is_normal() {
            // do a normal merge
            let head_commit = repo.reference_to_annotated_commit(&repo.head()?)?;
            normal_merge(&repo, &head_commit, &fetch_commit)?;
        } else {
            info!("Nothing to do!");
        }
    }

    Ok(())
}
