use std::fs;

use crate::{
    common::{constants, errors::VersionsError},
    types::{
        modules_config::{write_modules_config, ModulesConfig},
        repository::Repository,
    },
};

pub fn initialize_repository_filesystem(repository: &Repository) -> Result<(), VersionsError> {
    let repository_dir_path = repository.root_path.join(constants::REPOSITORY_DIR);
    fs::create_dir_all(repository_dir_path)?;
    write_modules_config(
        repository,
        &ModulesConfig {
            modules: Vec::new(),
        },
    )?;
    Ok(())
}
