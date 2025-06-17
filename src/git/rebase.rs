use git2::{BranchType, Oid, Rebase, Repository, Signature};

pub fn existing_rebase(repo: &Repository) -> Option<Rebase> {
    repo.open_rebase(None).ok()
}

fn next_commit(repo: &Repository, rebase: &mut Rebase, signature: &Signature) {
    let index = repo.index().unwrap();

    // TODO(James): conflicts will happen here - we want to display this error nicely to the user
    if index.has_conflicts() {
        panic!("conflict");
    }

    match rebase.commit(None, signature, None) {
        Ok(_) => {}
        // libgit2 counts this is an error, when really there's no issue
        Err(e) if e.message() == "this patch has already been applied" => {}
        Err(_) => {
            panic!("error while rebasing");
        }
    };
}

fn progress_rebase(repo: &Repository, rebase: &mut Rebase) {
    let signature = repo.signature().unwrap();

    while let Some(_) = rebase.next() {
        next_commit(repo, rebase, &signature);
    }

    // We need a final commit here for some reason
    next_commit(repo, rebase, &signature);

    rebase.finish(None).unwrap();
}

pub fn init_rebase(
    repo: &Repository,
    branch_name: impl AsRef<str>,
    parent_name: impl AsRef<str>,
    parent_known_revision: String,
) {
    let branch_ref = repo
        .find_branch(branch_name.as_ref(), BranchType::Local)
        .unwrap()
        .into_reference();

    let onto_ref = repo
        .find_branch(parent_name.as_ref(), BranchType::Local)
        .unwrap()
        .into_reference();

    let upstream_commit = repo
        .find_annotated_commit(Oid::from_str(parent_known_revision.as_str()).unwrap())
        .unwrap();

    let branch_commit = repo.reference_to_annotated_commit(&branch_ref).unwrap();
    let onto_commit = repo.reference_to_annotated_commit(&onto_ref).unwrap();

    let mut rebase = repo
        .rebase(
            Some(&branch_commit),
            Some(&upstream_commit),
            Some(&onto_commit),
            None,
        )
        .unwrap();

    progress_rebase(repo, &mut rebase);
}

pub fn continue_rebase(repo: &Repository) {
    let mut rebase = repo.open_rebase(None).unwrap();

    progress_rebase(repo, &mut rebase);
}
