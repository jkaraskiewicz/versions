use std::path::PathBuf;

use commons::utils::file_util;

use crate::{
    common::{
        constants,
        errors::VersionsError,
        git_util::{initialize_repository, repository_exists},
    },
    types::repository::Repository,
};

use super::filesystem_handler::initialize_repository_filesystem;

pub fn init(path: &PathBuf) -> Result<Repository, VersionsError> {
    if repository_exists(path)? && exists(path) {
        return Err(VersionsError::RepositoryAlreadyInitialized);
    }
    initialize_repository(path)?;
    let repository = Repository {
        root_path: path.to_path_buf(),
    };
    initialize_repository_filesystem(&repository)?;
    Ok(repository)
}

pub fn open(path: &PathBuf) -> Result<Repository, VersionsError> {
    if repository_exists(path)? && exists(path) {
        Ok(Repository {
            root_path: path.to_path_buf(),
        })
    } else {
        Err(VersionsError::RepositoryNotFound)
    }
}

pub fn exists(path: &PathBuf) -> bool {
    let repository_dir_path = path.join(constants::REPOSITORY_DIR);
    file_util::exists_directory(&repository_dir_path)
}
