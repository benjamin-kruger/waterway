use git2::Repository;

pub fn checkout(repo: &Repository, branch: &str) {
    repo.set_head(format!("refs/heads/{}", branch).as_str())
        .unwrap();

    repo.checkout_head(None).unwrap();
}

pub fn new(repo: &Repository, branch: &str) {
    let head_commit = repo.head().unwrap().peel_to_commit().unwrap();

    repo.branch(branch, &head_commit, false).unwrap();
}
