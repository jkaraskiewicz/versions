use commons::utils::file_util::{read_file, write_file};
use serde::{Deserialize, Serialize};

use crate::common::{
    constants,
    errors::VersionsError,
    git_util::{add_all, commit, force_get_repository},
    message_util::generate_update_message,
};

use super::{module::Module, repository::Repository};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModulesConfig {
    pub modules: Vec<Module>,
}

pub fn read_modules_config(repository: &Repository) -> Result<ModulesConfig, VersionsError> {
    let path = repository
        .root_path
        .join(constants::REPOSITORY_DIR)
        .join(constants::MODULES_FILE);
    let content = read_file(&path).map_err(|_| VersionsError::RepositoryNotInitialized)?;
    let config: ModulesConfig =
        toml::from_str(&content).map_err(|_| VersionsError::RepositoryNotInitialized)?;
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
    let content = toml::to_string(config)?;
    write_file(&path, &content)?;

    let repo = force_get_repository(&repository.root_path)?;
    add_all(&repo)?;
    commit(&repo, &generate_update_message())?;
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
