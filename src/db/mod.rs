use crate::git;
use std::{collections::HashMap, path::PathBuf};

use git2::Repository;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct BranchName(pub String);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Commit(pub String);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParentBranch {
    pub name: BranchName,
    pub known_revision: Commit,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GitBranch {
    pub name: BranchName,
    pub message: String,
    pub revision: Commit,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Branch {
    pub name: BranchName,
    pub git: GitBranch,
    pub children: Vec<BranchName>,
    pub parent: Option<ParentBranch>, // The branch with no parent is the trunk branch
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DB {
    pub branches: HashMap<BranchName, Branch>,
}

pub struct Metadata {
    path: PathBuf,
    db: DB,
}

impl Metadata {
    fn path(repo: &Repository) -> PathBuf {
        repo.path().join("waterway.json")
    }

    pub fn exists(repo: &Repository) -> bool {
        Metadata::path(repo).exists()
    }

    pub fn init(repo: &Repository, trunk: String) {
        let path = Metadata::path(repo);

        std::fs::File::create(&path).unwrap();

        let mut branches = HashMap::new();

        let trunk_commit = repo
            .find_reference(format!("refs/heads/{}", &trunk).as_str())
            .unwrap()
            .peel_to_commit()
            .unwrap();

        let name = BranchName(trunk);
        branches.insert(
            name.clone(),
            Branch {
                name: name.clone(),
                git: GitBranch {
                    name: name.clone(),
                    message: trunk_commit.message().unwrap().to_string(),
                    revision: Commit(trunk_commit.id().to_string()),
                },
                children: Vec::with_capacity(0),
                parent: None,
            },
        );

        Metadata {
            path,
            db: DB { branches },
        }
        .commit();
    }

    pub fn open(repo: &Repository) -> Self {
        let path = Metadata::path(repo);
        let raw = std::fs::read_to_string(&path).unwrap();
        let db: DB = serde_json::from_str(&raw).unwrap();

        Metadata { path, db }
    }

    pub fn commit(&self) {
        let raw = serde_json::to_string(&self.db).unwrap();
        std::fs::write(&self.path, raw).unwrap();
    }

    pub fn insert(&mut self, branch: Branch) {
        let branch_name = branch.name.clone();
        let parent_branch_name = branch.parent.clone().unwrap().name;

        self.db.branches.insert(branch_name.clone(), branch);
        self.db
            .branches
            .get_mut(&parent_branch_name)
            .unwrap()
            .children
            .push(branch_name.clone());

        self.commit();
    }

    pub fn revise(&mut self, git: GitBranch) {
        let name = git.name.clone();
        self.db.branches.get_mut(&name).unwrap().git = git;
        self.commit();
    }
}

impl Branch {
    pub fn from(parent: git::branch::Branch, branch: git::branch::Branch) -> Branch {
        Branch {
            name: BranchName(branch.name.clone()),
            git: GitBranch::from(branch),
            children: Vec::with_capacity(0),
            parent: Some(ParentBranch {
                name: BranchName(parent.name.clone()),
                known_revision: Commit(parent.commit.id().to_string()),
            }),
        }
    }
}

impl GitBranch {
    pub fn from(branch: git::branch::Branch) -> GitBranch {
        GitBranch {
            name: BranchName(branch.name.clone()),
            message: branch.commit.message().unwrap().to_string(),
            revision: Commit(branch.commit.id().to_string()),
        }
    }
}
