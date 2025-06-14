use git2::Repository;

use rand::{Rng, distr::Alphanumeric, rng};
use rstest::fixture;

const SUFFIX_LEN: usize = 12;

#[fixture]
pub fn repo() -> Repository {
    let repo_suffix =
        String::from_utf8(rng().sample_iter(&Alphanumeric).take(SUFFIX_LEN).collect())
            .expect("could not create suffix path");

    let repo = format!("/tmp/waterway/{}", repo_suffix);

    println!("creating repo at {}", &repo);

    Repository::init(repo).expect("could not initialise mock git repo")
}

pub fn initial_commit(repo: &Repository) {
    let tree_id = repo.index().unwrap().write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();

    let author = &repo.signature().unwrap();
    let committer = author;

    repo.commit(Some("HEAD"), author, committer, "init", &tree, &[])
        .unwrap();
}
