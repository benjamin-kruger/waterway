use git2::Repository;
use rstest::rstest;

use crate::common::repo;

#[rstest]
fn test_init_repo(repo: Repository) {
    assert_eq!(repo.is_empty().unwrap(), true);
    assert_eq!(repo.is_bare(), false);

    // TODO(James): test that waterway can init from the mock_repo
}
