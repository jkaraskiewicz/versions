use std::path::Path;

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
    repository_dir_path.is_dir()
}
