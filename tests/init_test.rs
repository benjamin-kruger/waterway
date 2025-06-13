mod common;

use git2::Repository;
use rstest::rstest;

use common::mock_repo;

#[rstest]
fn test_init_repo(mock_repo: Repository) {
    assert_eq!(mock_repo.is_empty().unwrap(), true);
    assert_eq!(mock_repo.is_bare(), false);

    // TODO(James): test that waterway can init from the mock_repo
}
