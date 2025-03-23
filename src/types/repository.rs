use super::{
    module::{create_default, Module},
    modules_config::{read_modules_config, update_modules_config},
};
use crate::common::errors::VersionsError;
use commons::utils::file_util::exists_directory;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Repository {
    pub root_path: PathBuf,
}

pub fn from_path(path: &PathBuf) -> Repository {
    Repository {
        root_path: path.to_path_buf(),
    }
}

impl Repository {
    fn get_module(&self, module_name: &str) -> Result<Module, VersionsError> {
        let modules_config = read_modules_config(self)?;
        let result = modules_config
            .modules
            .iter()
            .find(|m| m.name == module_name);
        match result {
            Some(module) => Ok(module.to_owned()),
            None => Err(VersionsError::ModuleDoesNotExists(module_name.to_string())),
        }
    }

    fn add_module(&self, name: &str, path: &PathBuf) -> Result<Module, VersionsError> {
        if !exists_directory(path) {
            return Err(VersionsError::NotADirectory);
        }
        if is_module_defined(self, name)? {
            return Err(VersionsError::ModuleAlreadyExists(name.to_string()));
        }
        let new_module = create_default(name, path, self);
        update_modules_config(self, |mut config| {
            config.modules.push(new_module.to_owned());
            config
        })?;
        Ok(new_module)
    }

    fn remove_module(&self, module: &Module) -> Result<(), VersionsError> {
        let module_dir_path = self.root_path.join(&module.directory);
        if !exists_directory(&module_dir_path) {
            return Err(VersionsError::NotADirectory);
        }
        if !is_module_defined(self, &module.name)? {
            return Err(VersionsError::ModuleDoesNotExists(module.name.to_string()));
        }
        update_modules_config(self, |mut config| {
            config.modules.retain(|m| m.uid != module.uid);
            config
        })?;
        Ok(())
    }

    fn list_modules(&self) -> Result<Vec<Module>, VersionsError> {
        let modules_config = read_modules_config(self)?;
        Ok(modules_config.modules)
    }
}

fn is_module_defined(repository: &Repository, name: &str) -> Result<bool, VersionsError> {
    let config = read_modules_config(repository)?;
    let result = config.modules.iter().find(|el| el.name == name).is_some();
    Ok(result)
}
