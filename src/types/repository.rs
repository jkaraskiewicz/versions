use super::{
    module::Module,
    modules_config::{read_modules_config, update_modules_config},
};
use crate::common::{
    errors::VersionsError,
    module_util::{create_default, is_module_defined},
};
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
        if !path.as_ref().is_dir() {
            return Err(VersionsError::NotADirectory);
        }
        if is_module_defined(self, name)? {
            return Err(VersionsError::ModuleAlreadyExists(name.to_string()));
        }
        let new_module = create_default(self, name, path.as_ref());
        update_modules_config(self, |mut config| {
            config.modules.push(new_module.to_owned());
            if config.current_module.is_none() {
                config.current_module = Some(new_module.to_owned());
            }
            config
        })?;
        if let Some(current_version) = &new_module.current_version {
            current_version.save()?;
        }
        Ok(new_module)
    }

    pub fn select_module(&self, module: &Module) -> Result<Module, VersionsError> {
        update_modules_config(self, |mut config| {
            config.current_module = Some(module.to_owned());
            config
        })?;
        Ok(module.to_owned())
    }

    pub fn remove_module(&self, module: &Module) -> Result<(), VersionsError> {
        let module_dir_path = self.root_path.join(&module.directory);
        if !module_dir_path.is_dir() {
            return Err(VersionsError::NotADirectory);
        }
        if !is_module_defined(self, &module.name)? {
            return Err(VersionsError::ModuleDoesNotExists(module.name.to_string()));
        }

        for version in &module.versions {
            version.remove()?;
        }

        update_modules_config(self, |mut config| {
            config.modules.retain(|m| m.name != module.name);
            if let Some(current_module) = &config.current_module {
                if current_module.name == module.name {
                    config.current_module = None;
                }
            }
            config
        })?;
        Ok(())
    }

    pub fn current_module(&self) -> Result<Option<Module>, VersionsError> {
        let modules_config = read_modules_config(self)?;
        Ok(modules_config.current_module)
    }

    pub fn force_current_module(&self) -> Result<Module, VersionsError> {
        let modules_config = read_modules_config(self)?;
        if let Some(current_module) = &modules_config.current_module {
            Ok(current_module.to_owned())
        } else {
            Err(VersionsError::NoCurrentModule)
        }
    }

    pub fn list_modules(&self) -> Result<Vec<Module>, VersionsError> {
        let modules_config = read_modules_config(self)?;
        Ok(modules_config.modules)
    }

    pub fn save_workspace(&self) -> Result<(), VersionsError> {
        let modules = self.list_modules()?;
        for module in modules {
            if let Some(current_version) = &module.current_version {
                current_version.save()?;
            }
        }
        Ok(())
    }

    pub fn load_workspace(&self) -> Result<(), VersionsError> {
        let modules = self.list_modules()?;
        for module in modules {
            if let Some(current_version) = &module.current_version {
                current_version.load()?;
            }
        }
        Ok(())
    }
}
