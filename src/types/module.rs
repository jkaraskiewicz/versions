use super::{
    meta::{ModulePtr, RepositoryPtr},
    modules_config::{update_module_in_config, update_modules_config},
    version::Version,
};
use crate::common::{errors::VersionsError, repository_util::from_path};
use commons::traits::collections::{Contains, FirstItemPredicate};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Module {
    #[serde(skip)]
    pub repository_ptr: RepositoryPtr,
    pub name: String,
    pub directory: String,
    pub versions: Vec<Version>,
    pub current_version: Option<Version>,
}

impl Module {
    pub fn list_versions(&self) -> Vec<Version> {
        self.versions.to_owned()
    }

    pub fn add_version(&mut self, name: &str) -> Result<Version, VersionsError> {
        let new_version = Version {
            name: name.to_string(),
            module: ModulePtr::create(self),
        };
        let already_exists = self.versions.contains(&new_version);
        if already_exists {
            Err(VersionsError::VersionAlreadyExists(name.to_string()))
        } else {
            self.versions.push(new_version.to_owned());
            update_modules_config(
                &from_path(&self.repository_ptr.repository_path),
                |mut config| {
                    config.modules.retain(|el| el.name != self.name);
                    config.modules.push(self.to_owned());
                    config
                },
            )?;
            new_version.save()?;
            Ok(new_version.to_owned())
        }
    }

    pub fn remove_version(&mut self, name: &str) -> Result<(), VersionsError> {
        let version = self
            .versions
            .first(|el| el.name == name)
            .map(|v| v.to_owned());
        if let Some(version) = version {
            version.remove()?;
            self.versions.retain(|el| el.name != name);

            if let Some(current_version) = &self.current_version {
                if current_version.name == version.name {
                    self.current_version = None;
                }
            }

            update_modules_config(
                &from_path(&self.repository_ptr.repository_path),
                |mut config| {
                    config.modules.retain(|el| el.name != self.name);
                    config.modules.push(self.to_owned());
                    config
                },
            )?;
            Ok(())
        } else {
            Err(VersionsError::VersionDoesNotExists(name.to_string()))
        }
    }

    pub fn select_version(&mut self, name: &str) -> Result<Version, VersionsError> {
        let version = self.versions.iter().find(|version| version.name == name);
        if let Some(version) = version {
            if let Some(current_version) = &self.current_version {
                current_version.save()?;
            }
            self.current_version = Some(version.to_owned());
            version.load()?;
            update_module_in_config(&from_path(&self.repository_ptr.repository_path), self)?;
            Ok(version.to_owned())
        } else {
            Err(VersionsError::VersionDoesNotExists(name.to_string()))
        }
    }

    pub fn deselect_version(&mut self) -> Result<(), VersionsError> {
        if let Some(current_version) = &self.current_version {
            current_version.save()?;
            self.current_version = None;
            update_module_in_config(&from_path(&self.repository_ptr.repository_path), self)?;
        };
        Ok(())
    }

    pub fn current_version(&self) -> Result<Option<Version>, VersionsError> {
        Ok(self.current_version.to_owned())
    }

    pub fn force_current_version(&self) -> Result<Version, VersionsError> {
        if let Some(current_version) = &self.current_version {
            Ok(current_version.to_owned())
        } else {
            Err(VersionsError::NoCurrentVersionInModule(
                self.name.to_string(),
            ))
        }
    }
}
