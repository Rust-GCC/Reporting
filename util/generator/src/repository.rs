//! A module providing utility functions to interact with git repositories
use crate::naming::{ORGANISATION, REPOSITORY};
use crate::progress::{default_progress_bar, FetchMethod, ProgressBarExt, Steps};
use git2::build::RepoBuilder;
use git2::{Error, FetchOptions, RemoteCallbacks, Repository, RepositoryInitMode};
use indicatif::{MultiProgress, ProgressBar};
use octocrab::Octocrab;
use std::fmt;
use tokio::task::{self, JoinError};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs::create_dir;
use std::io::Result as IOResult;
use std::path::PathBuf;

/// Creates a new directory in temporary ressources folder.
fn rand_dir() -> IOResult<PathBuf> {
    let mut length = 5;
    loop {
        let tmp_path = std::env::temp_dir().join(
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(length)
                .map(char::from)
                .collect::<String>(),
        );

        if !tmp_path.exists() {
            break create_dir(tmp_path.clone()).map(|_| tmp_path);
        } else {
            length += 1;
        }
    }
}

/// Set of error that might arise during a clone operation.
#[derive(Debug)]
pub enum CloneError {
    /// Github API provided no clone URL.
    MissingCloneUrl,
    /// Met a git2 error during the actual clone action.
    Download(Error),
    /// Error during workspace/directory setup.
    Local(std::io::Error),
    /// Thread related error.
    Thread(JoinError),
    /// Met an octocrab error whilst retrieving repository data.
    Octocrab(octocrab::Error),
}

impl fmt::Display for CloneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error happened while cloning repository")
    }
}

impl std::error::Error for CloneError {}

/// Clone gccrs repository to the systems's temp directory.
///
/// # Arguments
///
/// * `gh` - Reference to an octocrab instance.
pub async fn clone(gh: &Octocrab, multi: &MultiProgress) -> Result<Repository, CloneError> {
    use CloneError::{Download, Local, MissingCloneUrl, Octocrab, Thread};
    let gh_repository = gh
        .repos(ORGANISATION, REPOSITORY)
        .get()
        .await
        .map_err(Octocrab)?;

    let clone_url = gh_repository.clone_url.ok_or(MissingCloneUrl)?;

    let workspace = rand_dir().map_err(Local)?;

    let bar = default_progress_bar();
    multi.add(bar.clone());

    let clone_destination = workspace.clone();
    let res = task::spawn_blocking(move || {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.transfer_progress(|p| {
            bar.set_length(p.total_objects().try_into().unwrap_or_default());
            bar.set_position(p.indexed_objects().try_into().unwrap_or_default());
            bar.set_message("Downloading");
            true
        });
        let mut fo = FetchOptions::new();
        fo.remote_callbacks(callbacks);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fo);
        builder
            .clone(clone_url.as_str(), &clone_destination)
            .map_err(Download)
    })
    .await
    .map_err(Thread)?;
    res
}

/// Checkout a given repository to a given reference.
///
/// The reference can be either a commit, a branch name or a tag.
///
/// # Arguments
///
/// `repo` - The repository to checkout.
/// `refname` - The name of the reference to checkout to.
pub fn checkout(repo: &Repository, refname: &str) -> Result<(), Error> {
    let (object, reference) = repo.revparse_ext(refname)?;

    repo.checkout_tree(&object, None)?;

    match reference {
        // branch or tag
        Some(gref) => repo.set_head(gref.name().unwrap()),
        // commit
        None => repo.set_head_detached(object.id()),
    }
}
