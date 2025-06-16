use git2::{Commit, Repository};

pub struct Branch<'repo> {
    pub name: String,
    pub commit: Commit<'repo>,
}

pub fn head(repo: &Repository) -> Branch {
    let head = repo.head().unwrap();

    if !head.is_branch() {
        panic!("head is not pointing at a branch");
    }

    let name = head.shorthand().unwrap().to_string();
    let commit = head.peel_to_commit().unwrap();

    Branch { name, commit }
}

pub fn checkout(repo: &Repository, branch: &str) {
    repo.set_head(format!("refs/heads/{}", branch).as_str())
        .unwrap();

    repo.checkout_head(None).unwrap();
}

pub fn new(repo: &Repository, branch: &str) {
    let head_commit = repo.head().unwrap().peel_to_commit().unwrap();

    repo.branch(branch, &head_commit, false).unwrap();
}
