use std::path::Path;

use git2::Repository;

pub fn init_repo(path: impl AsRef<Path>) -> Repository {
    Repository::init(path).expect("could not initialise repo")
}

pub fn open_repo(path: impl AsRef<Path>) -> Repository {
    Repository::open(path).expect("could not open repo from current env")
}
