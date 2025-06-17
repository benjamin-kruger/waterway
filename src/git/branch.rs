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

pub fn from(repo: &Repository, branch: impl AsRef<str>) -> Branch {
    let branch_unqualified = branch
        .as_ref()
        .strip_prefix("refs/heads/")
        .unwrap_or(branch.as_ref());

    let reference = repo
        .find_branch(branch_unqualified, git2::BranchType::Local)
        .unwrap()
        .into_reference();

    if !reference.is_branch() {
        panic!("branch name is not pointing at a branch");
    }

    let commit = reference.peel_to_commit().unwrap();

    Branch {
        name: branch_unqualified.to_string(),
        commit,
    }
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
