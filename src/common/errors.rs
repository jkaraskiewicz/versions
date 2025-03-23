use std::io;

use thiserror::Error;
use toml::ser;

#[derive(Error, Debug)]
pub enum VersionsError {
    #[error("I/O error: `{0}`")]
    IoError(#[from] io::Error),
    #[error("Toml error: `{0}`")]
    SerdeTomlError(#[from] ser::Error),
    #[error("Commons error: `{0}`")]
    CommonsError(#[from] commons::types::errors::CommonsError),
    #[error("Git error: `{0}`")]
    GitError(#[from] git2::Error),
    #[error("Repository not found")]
    RepositoryNotFound,
    #[error("Repository already initialized")]
    RepositoryAlreadyInitialized,
    #[error("Repository not initialized")]
    RepositoryNotInitialized,
    #[error("Not a directory")]
    NotADirectory,
    #[error("Module `{0}` already exists")]
    ModuleAlreadyExists(String),
    #[error("Module `{0}` does not exist")]
    ModuleDoesNotExists(String),
    #[error("Branch `{0}` already exists")]
    BranchAlreadyExists(String),
    #[error("Branch `{0}` does not exist")]
    BranchDoesNotExists(String),
}
