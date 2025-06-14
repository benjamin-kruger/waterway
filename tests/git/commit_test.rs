use std::io::Write;

use git2::Repository;
use rstest::rstest;

use crate::common::{initial_commit, repo};

use waterway::git;

#[rstest]
fn test_initial_commit(repo: Repository) {
    let workdir = repo.workdir().unwrap();
    let mock_file = workdir.join("mock.txt");

    let mut file = std::fs::File::create_new(mock_file).unwrap();
    file.write_all(b"mock data").unwrap();

    git::index::add_all(&repo);
    initial_commit(&repo);

    file.write(b"extra data").unwrap();
    git::commit::new(&repo, "test");
}
