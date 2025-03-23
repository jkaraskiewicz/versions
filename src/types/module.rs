use super::{
    branch::Branch,
    modules_config::update_modules_config,
    repository::{from_path, Repository},
};
use crate::common::errors::VersionsError;
use commons::{traits::vec_traits::Contains, utils::random_util::generate_uid};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Module {
    pub uid: String,
    pub name: String,
    pub directory: String,
    pub branches: Vec<Branch>,
    pub current_branch: Branch,
}

impl Module {
    pub fn list_branches(&self) -> Result<Vec<Branch>, VersionsError> {
        Ok(self.branches.to_owned())
    }

    pub fn add_branch(mut self, name: &str) -> Result<Branch, VersionsError> {
        let new_branch = Branch {
            name: name.to_string(),
        };
        let already_exists = self.branches.has(&new_branch);
        if already_exists {
            Err(VersionsError::BranchAlreadyExists(name.to_string()))
        } else {
            self.branches.push(new_branch.to_owned());
            update_modules_config(&from_path(&self.repository_path), |mut config| {
                config.modules.retain(|el| el.uid != self.uid);
                config.modules.push(self.to_owned());
                config
            })?;
            Ok(new_branch.to_owned())
        }
    }

    pub fn remove_branch(mut self, name: &str) -> Result<(), VersionsError> {
        let new_branch = Branch {
            name: name.to_string(),
        };
        let already_exists = self.branches.has(&new_branch);
        if !already_exists {
            Err(VersionsError::BranchDoesNotExists(name.to_string()))
        } else {
            self.branches.retain(|el| el.name != name);
            update_modules_config(&from_path(&self.repository_path), |mut config| {
                config.modules.retain(|el| el.uid != self.uid);
                config.modules.push(self.to_owned());
                config
            })?;
            Ok(())
        }
    }

    pub fn get_branch(&self, name: &str) -> Result<Branch, VersionsError> {
        let branch = self.branches.iter().find(|branch| branch.name == name);
        match branch {
            Some(branch) => Ok(branch.to_owned()),
            None => Err(VersionsError::BranchDoesNotExists(name.to_string())),
        }
    }

    pub fn current_branch(&self) -> Result<Branch, VersionsError> {
        Ok(self.current_branch.to_owned())
    }
}

pub fn create_default(name: &str, dir_path: &PathBuf, repository: &Repository) -> Module {
    let branch = Branch {
        name: "default".to_string(),
    };
    let result = Module {
        uid: generate_uid(&format!("{}{}", name, dir_path.to_str().unwrap())),
        name: name.to_string(),
        directory: dir_path.file_name().unwrap().to_str().unwrap().to_string(),
        branches: vec![branch.to_owned()],
        current_branch: branch.to_owned(),
    };
    let _ = update_modules_config(repository, |mut config| {
        config.modules.push(result.to_owned());
        config
    });
    result
}
