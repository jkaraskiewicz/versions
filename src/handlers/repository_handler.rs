use std::path::Path;

use commons::utils::file_util;

use crate::{
    common::{constants, errors::VersionsError, repository_util::from_path},
    types::repository::Repository,
};

use super::filesystem_handler::initialize_repository_filesystem;

pub fn init<P: AsRef<Path>>(path: P) -> Result<Repository, VersionsError> {
    if exists(path.as_ref()) {
        return Err(VersionsError::RepositoryAlreadyInitialized);
    }
    let repository = from_path(path.as_ref());
    initialize_repository_filesystem(&repository)?;
    Ok(repository)
}

pub fn open<P: AsRef<Path>>(path: P) -> Result<Repository, VersionsError> {
    if exists(path.as_ref()) {
        Ok(from_path(path.as_ref()))
    } else {
        Err(VersionsError::RepositoryNotFound)
    }
}

pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    let repository_dir_path = path.as_ref().join(constants::REPOSITORY_DIR);
    file_util::exists_directory(&repository_dir_path)
}
