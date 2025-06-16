use std::path::Path;

use git2::Repository;

pub fn ensure_initial_commit(repo: &Repository) {
    if repo.head().is_err() {
        panic!("expected HEAD to exist - please create an initial commit")
    }
}

pub fn open_repo(path: impl AsRef<Path>) -> Repository {
    let repo = Repository::open(path).expect("could not open repo from current env");

    ensure_initial_commit(&repo);

    repo
}
