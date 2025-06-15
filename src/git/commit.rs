use git2::Repository;

pub fn new(repo: &Repository, message: &str) {
    let tree_id = repo.index().unwrap().write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();

    // TODO(James): signature will fail if git config user.email and user.name is not set
    let author = &repo.signature().unwrap();
    let committer = author;

    let head_commit = repo.head().unwrap().peel_to_commit().unwrap();
    let parents = [&head_commit];

    repo.commit(Some("HEAD"), author, committer, message, &tree, &parents)
        .unwrap();
}

pub fn amend(repo: &Repository, message: Option<&str>) {
    let tree_id = repo.index().unwrap().write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();

    let head_ref = repo.head().unwrap();
    let head_commit = head_ref.peel_to_commit().unwrap();

    head_commit
        .amend(
            Some(&head_ref.name().unwrap()),
            None,
            None,
            None,
            Some(message.unwrap_or(head_commit.message().unwrap())),
            Some(&tree),
        )
        .unwrap();
}
