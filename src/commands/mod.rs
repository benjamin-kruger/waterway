use crate::git;
use std::path::Path;

pub fn init(dir: impl AsRef<Path>) {
    git::init::init_repo(dir);
}

pub fn create(dir: impl AsRef<Path>, message: String) {
    let repo = git::init::open_repo(dir);

    if git::util::has_staged_changes(&repo) {
        let branch: String = message
            .as_str()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect();

        git::branch::new(&repo, branch.as_str());
        git::branch::checkout(&repo, branch.as_str());

        git::commit::new(&repo, message.as_ref());
    } else {
        println!("No staged changes");
    }
}

pub fn modify(dir: impl AsRef<Path>, message: Option<String>) {
    let repo = git::init::open_repo(dir);

    if git::util::has_staged_changes(&repo) {
        git::commit::amend(&repo, message.as_deref());
    } else {
        println!("No staged changes");
    }
}
