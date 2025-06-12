use git2::Repository;
use rand::{Rng, distr::Alphanumeric, rng};
use rstest::fixture;

const SUFFIX_LEN: usize = 12;

#[fixture]
pub fn mock_repo() -> Repository {
    let repo_suffix =
        String::from_utf8(rng().sample_iter(&Alphanumeric).take(SUFFIX_LEN).collect())
            .expect("could not create suffix path");

    let repo = format!("/tmp/waterway/{}", repo_suffix);

    println!("creating repo at {}", &repo);

    Repository::init(repo).expect("could not initialise mock git repo")
}
