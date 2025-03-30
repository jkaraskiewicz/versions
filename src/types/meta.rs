use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::{module::Module, repository::Repository};

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct RepositoryPtr {
    pub repository_path: PathBuf,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct ModulePtr {
    pub repository_path: PathBuf,
    pub module_name: String,
    pub module_dir: String,
}

impl RepositoryPtr {
    pub fn create(repository: &Repository) -> Self {
        RepositoryPtr {
            repository_path: repository.root_path.to_path_buf(),
        }
    }
}

impl ModulePtr {
    pub fn create(module: &Module) -> Self {
        ModulePtr {
            repository_path: module.repository_ptr.repository_path.to_path_buf(),
            module_name: module.name.to_string(),
            module_dir: module.directory.to_string(),
        }
    }
}
