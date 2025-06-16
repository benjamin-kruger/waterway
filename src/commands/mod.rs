use crate::{
    db::{self, Metadata},
    git,
};
use git2::Repository;
use std::path::Path;

pub fn init(dir: impl AsRef<Path>, trunk: String) {
    let repo = git::init::open_repo(dir);

    db::Metadata::init(&repo, trunk);
}

pub struct Context {
    repo: Repository,
    meta: Metadata,
}

fn ensure_init(dir: impl AsRef<Path>) -> Context {
    let repo = git::init::open_repo(dir);

    if !db::Metadata::exists(&repo) {
        // TODO(James): prompt the user to init automatically if not already initialised
        panic!("waterway is not initialised - please initialise with `waterway init`")
    }

    let meta = db::Metadata::open(&repo);

    Context { repo, meta }
}

pub fn create(dir: impl AsRef<Path>, message: String) {
    let Context { repo, mut meta } = ensure_init(dir);

    if git::util::has_staged_changes(&repo) {
        let branch_name: String = message
            .as_str()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect();

        let parent = git::branch::head(&repo);

        git::branch::new(&repo, branch_name.as_str());
        git::branch::checkout(&repo, branch_name.as_str());

        git::commit::new(&repo, message.as_ref());

        let branch = git::branch::head(&repo);

        let db_branch = db::Branch::from(parent, branch);
        meta.insert(db_branch);
    } else {
        println!("No staged changes");
    }
}

pub fn modify(dir: impl AsRef<Path>, message: Option<String>) {
    let Context { repo, mut meta } = ensure_init(dir);

    if git::util::has_staged_changes(&repo) {
        git::commit::amend(&repo, message.as_deref());

        let branch = git::branch::head(&repo);
        meta.revise(db::GitBranch::from(branch));
    } else {
        println!("No staged changes");
    }
}
