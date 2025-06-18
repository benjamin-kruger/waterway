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
        let branch_name = branch.name.clone();

        meta.revise(db::GitBranch::from(branch));

        restack_recursive(&repo, &mut meta, Some(branch_name), false);
    } else {
        println!("No staged changes");
    }
}

enum RestackOperation {
    Init,
    Continue,
}

fn restack_one(
    repo: &Repository,
    meta: &mut Metadata,
    db_branch: &db::Branch,
    op: RestackOperation,
) {
    let parent = if let Some(p) = db_branch.parent.clone() {
        p
    } else {
        // trunk branch does not need to be restacked?
        return;
    };

    let db_parent = meta.get(&parent.name).unwrap();

    if parent.known_revision == db_parent.git.revision {
        println!(
            "{} does not need to be rebased onto {}",
            db_branch.name.0.as_str(),
            db_parent.name.0.as_str()
        );
        // branch does not need to be restacked
        return;
    }

    match op {
        RestackOperation::Init => {
            git::rebase::init_rebase(
                repo,
                db_branch.name.0.as_str(),
                parent.name.0.as_str(),
                parent.known_revision.0,
            );
        }
        RestackOperation::Continue => {
            git::rebase::continue_rebase(repo);
        }
    }

    let updated_parent = git::branch::from(repo, parent.name.0.as_str());

    meta.update_parent(
        &db_branch.name,
        db::ParentBranch {
            known_revision: db::Commit::from(updated_parent.commit),
            ..parent
        },
    );
}

pub fn restack_recursive(
    repo: &Repository,
    meta: &mut Metadata,
    branch_name: Option<String>,
    cont: bool,
) {
    let db_branch = if let Some(rebase) = git::rebase::existing_rebase(&repo) {
        let branch = git::branch::from(&repo, rebase.orig_head_name().unwrap());
        let db_branch = meta.get(&db::BranchName(branch.name)).unwrap();

        if cont {
            restack_one(&repo, meta, &db_branch, RestackOperation::Continue);
        } else {
            panic!(
                "rebase already in progress - use --continue flag to continue existing rebase operation."
            )
        }

        db_branch
    } else {
        let branch = if let Some(b) = branch_name {
            git::branch::from(&repo, b)
        } else {
            git::branch::head(&repo)
        };
        let db_branch = meta.get(&db::BranchName(branch.name)).unwrap();

        restack_one(&repo, meta, &db_branch, RestackOperation::Init);

        db_branch
    };

    for child_branch_name in db_branch.children.iter() {
        restack_recursive(repo, meta, Some(child_branch_name.0.clone()), false);
    }

    git::branch::checkout(&repo, db_branch.name.0.as_str());
}

pub fn restack(dir: impl AsRef<Path>, branch_name: Option<String>, cont: bool) {
    let Context { repo, mut meta } = ensure_init(dir);

    restack_recursive(&repo, &mut meta, branch_name, cont);
}
