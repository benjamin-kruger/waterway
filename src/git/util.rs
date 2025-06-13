use git2::Repository;

pub fn has_staged_changes(repo: &Repository) -> bool {
    let head_tree = repo.head().unwrap().peel_to_tree().unwrap();
    let index = repo.index().unwrap();

    repo.diff_tree_to_index(Some(&head_tree), Some(&index), None)
        .unwrap()
        .deltas()
        .next()
        .is_some()
}
