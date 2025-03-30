use super::{
    module::Module,
    modules_config::{read_modules_config, update_modules_config},
};
use crate::common::{
    errors::VersionsError,
    module_util::{create_default, is_module_defined},
};
use commons::utils::file_util::exists_directory;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Repository {
    pub root_path: PathBuf,
}

impl Repository {
    pub fn get_module(&self, module_name: &str) -> Result<Module, VersionsError> {
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

    pub fn add_module<P: AsRef<Path>>(&self, name: &str, path: P) -> Result<Module, VersionsError> {
        if !exists_directory(path.as_ref()) {
            return Err(VersionsError::NotADirectory);
        }
        if is_module_defined(self, name)? {
            return Err(VersionsError::ModuleAlreadyExists(name.to_string()));
        }
        let new_module = create_default(self, name, path.as_ref());
        update_modules_config(self, |mut config| {
            config.modules.push(new_module.to_owned());
            config
        })?;
        new_module.current_version.save()?;
        Ok(new_module)
    }

    pub fn remove_module(&self, module: &Module) -> Result<(), VersionsError> {
        let module_dir_path = self.root_path.join(&module.directory);
        if !exists_directory(&module_dir_path) {
            return Err(VersionsError::NotADirectory);
        }
        if !is_module_defined(self, &module.name)? {
            return Err(VersionsError::ModuleDoesNotExists(module.name.to_string()));
        }
        update_modules_config(self, |mut config| {
            config.modules.retain(|m| m.name != module.name);
            config
        })?;
        Ok(())
    }

    pub fn list_modules(&self) -> Result<Vec<Module>, VersionsError> {
        let modules_config = read_modules_config(self)?;
        Ok(modules_config.modules)
    }

    pub fn save_workspace(&self) -> Result<(), VersionsError> {
        let modules = self.list_modules()?;
        for module in modules {
            module.current_version()?.save()?;
        }
        Ok(())
    }

    pub fn load_workspace(&self) -> Result<(), VersionsError> {
        let modules = self.list_modules()?;
        for module in modules {
            module.current_version()?.load()?;
        }
        Ok(())
    }
}
