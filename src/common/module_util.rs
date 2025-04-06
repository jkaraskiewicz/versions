use std::path::Path;

use crate::types::{
    meta::{ModulePtr, RepositoryPtr},
    module::Module,
    modules_config::read_modules_config,
    repository::Repository,
    version::Version,
};

use super::{constants, errors::VersionsError};

pub fn is_module_defined(repository: &Repository, name: &str) -> Result<bool, VersionsError> {
    let config = read_modules_config(repository)?;
    let result = config.modules.iter().any(|el| el.name == name);
    Ok(result)
}

pub fn create_default(repository: &Repository, name: &str, dir_path: &Path) -> Module {
    let version = Version {
        name: constants::DEFAULT_VERSION.to_string(),
        module: ModulePtr {
            repository_path: repository.root_path.to_path_buf(),
            module_name: name.to_string(),
            module_dir: dir_path.file_name().unwrap().to_str().unwrap().to_string(),
        },
    };
    let result = Module {
        repository_ptr: RepositoryPtr::create(repository),
        name: name.to_string(),
        directory: dir_path.file_name().unwrap().to_str().unwrap().to_string(),
        versions: vec![version.to_owned()],
        current_version: Some(version.to_owned()),
    };
    result
}
