use crate::{
    common::errors::VersionsError,
    types::{
        module::{create_default, Module},
        modules_config::{read_modules_config, update_modules_config},
        repository::Repository,
    },
};
use std::path::PathBuf;

pub fn is_module_defined(repository: &Repository, name: &str) -> Result<bool, VersionsError> {
    let config = read_modules_config(repository)?;
    let result = config.modules.iter().find(|el| el.name == name).is_some();
    Ok(result)
}

pub fn add_module_with_default_config(
    repository: &Repository,
    dir_path: &PathBuf,
    name: &str,
) -> Result<Module, VersionsError> {
    if is_module_defined(repository, name)? {
        Err(VersionsError::ModuleAlreadyExists(name.to_string()))
    } else {
        let new_module = create_default(name, dir_path, repository);
        update_modules_config(repository, |mut config| {
            config.modules.push(new_module.to_owned());
            config
        })?;
        Ok(new_module)
    }
}

pub fn remove_module(repository: &Repository, module: &Module) -> Result<(), VersionsError> {
    if !is_module_defined(repository, &module.name)? {
        Err(VersionsError::ModuleDoesNotExists(module.name.to_string()))
    } else {
        update_modules_config(repository, |mut config| {
            config.modules.retain(|m| m.name != module.name);
            config
        })?;
        Ok(())
    }
}
