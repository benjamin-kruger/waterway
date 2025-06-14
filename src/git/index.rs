use git2::{IndexAddOption, Repository};

pub fn add_all(repo: &Repository) {
    // TODO(James): can we add some options here for including / excluding untracked files?
    let mut index = repo.index().unwrap();
    index
        .add_all(["*"], IndexAddOption::CHECK_PATHSPEC, None)
        .unwrap();
    index.write().unwrap();
}
