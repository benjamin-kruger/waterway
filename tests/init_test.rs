mod common;

use git2::Repository;
use rstest::rstest;

use common::blank_repo;

#[rstest]
fn test_init_repo(blank_repo: Repository) {
    assert_eq!(blank_repo.is_empty().unwrap(), true);
    assert_eq!(blank_repo.is_bare(), false);

    // TODO(James): test that waterway can init from the mock_repo
}
