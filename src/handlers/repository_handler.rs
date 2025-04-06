use std::path::{Path, PathBuf};

use crate::{
    common::{
        errors::VersionsError,
        repository_util::{from_path, hosts_repository},
    },
    types::repository::Repository,
};

use super::filesystem_handler::initialize_repository_filesystem;

pub fn init<P: AsRef<Path>>(path: P) -> Result<Repository, VersionsError> {
    if exists(path.as_ref(), false).is_some() {
        return Err(VersionsError::RepositoryAlreadyInitialized);
    }
    let repository = from_path(path.as_ref());
    initialize_repository_filesystem(&repository)?;
    Ok(repository)
}

pub fn open<P: AsRef<Path>>(path: P, look_up: bool) -> Result<Repository, VersionsError> {
    if let Some(repository_path) = exists(path, look_up) {
        Ok(from_path(repository_path))
    } else {
        Err(VersionsError::RepositoryNotFoundOrInitialized)
    }
}

pub fn exists<P: AsRef<Path>>(path: P, look_up: bool) -> Option<PathBuf> {
    if !look_up {
        return if hosts_repository(path.as_ref()) {
            Some(path.as_ref().to_path_buf())
        } else {
            None
        };
    };
    for ancestor in path.as_ref().ancestors() {
        if hosts_repository(ancestor) {
            return Some(ancestor.to_path_buf());
        };
    }
    None
}
