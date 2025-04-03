use super::{
    meta::{ModulePtr, RepositoryPtr},
    module::Module,
    repository::Repository,
    version::Version,
};
use crate::common::{constants, errors::VersionsError};
use commons::utils::file_util::{read_file, write_file};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModulesConfig {
    pub modules: Vec<Module>,
    pub current_module: Option<Module>,
}

pub fn read_modules_config(repository: &Repository) -> Result<ModulesConfig, VersionsError> {
    let path = repository
        .root_path
        .join(constants::REPOSITORY_DIR)
        .join(constants::MODULES_FILE);
    let content = read_file(&path).map_err(|_| VersionsError::RepositoryNotInitialized)?;
    let config: ModulesConfig =
        serde_yml::from_str(&content).map_err(|_| VersionsError::RepositoryNotInitialized)?;
    let config = append_metadata_to_config(repository, &config)?;
    Ok(config)
}

pub fn write_modules_config(
    repository: &Repository,
    config: &ModulesConfig,
) -> Result<(), VersionsError> {
    let path = repository
        .root_path
        .join(constants::REPOSITORY_DIR)
        .join(constants::MODULES_FILE);
    let content = serde_yml::to_string(config)?;
    write_file(&path, &content)?;
    Ok(())
}

pub fn update_modules_config(
    repository: &Repository,
    mut updater: impl FnMut(ModulesConfig) -> ModulesConfig,
) -> Result<ModulesConfig, VersionsError> {
    let config = read_modules_config(repository)?;

    let result = updater(config);

    write_modules_config(repository, &result)?;
    Ok(result)
}

pub fn update_module_in_config(
    repository: &Repository,
    module: &Module,
) -> Result<ModulesConfig, VersionsError> {
    let mut config = read_modules_config(repository)?;
    let mut modules = config.modules.to_vec();
    modules.retain(|m| m.name != module.name);

    modules.push(module.to_owned());
    config.modules = modules;

    write_modules_config(repository, &config)?;
    Ok(config)
}

fn append_metadata_to_config(
    repository: &Repository,
    config: &ModulesConfig,
) -> Result<ModulesConfig, VersionsError> {
    let modules: Vec<Module> = config
        .modules
        .iter()
        .map(|module| Module {
            repository_ptr: RepositoryPtr::create(repository),
            current_version: Version {
                name: module.current_version.name.to_string(),
                module: ModulePtr {
                    repository_path: repository.root_path.to_path_buf(),
                    module_name: module.name.to_string(),
                    module_dir: module.directory.to_string(),
                },
            },
            versions: module
                .versions
                .iter()
                .map(|version| Version {
                    name: version.name.to_string(),
                    module: ModulePtr {
                        repository_path: repository.root_path.to_path_buf(),
                        module_name: module.name.to_string(),
                        module_dir: module.directory.to_string(),
                    },
                })
                .collect(),
            ..module.to_owned()
        })
        .collect();

    let mut updated_current_module: Option<Module> = None;
    if let Some(current_module) = &config.current_module {
        updated_current_module = modules
            .iter()
            .find(|module| module.name == current_module.name)
            .map(|el| el.to_owned());
    }

    Ok(ModulesConfig {
        modules,
        current_module: updated_current_module,
    })
}
