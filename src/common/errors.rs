use std::{io, path::StripPrefixError};

use base64::DecodeError;
use thiserror::Error;
use toml::{de, ser};

#[derive(Error, Debug)]
pub enum VersionsError {
    #[error("I/O error: `{0}`")]
    IoError(#[from] io::Error),
    #[error("Toml serialization error: `{0}`")]
    TomlSerializationError(#[from] ser::Error),
    #[error("Toml deserialization error: `{0}`")]
    TomlDeserializationError(#[from] de::Error),
    #[error("Commons error: `{0}`")]
    CommonsError(#[from] commons::types::errors::CommonsError),
    #[error("Repository not found or not initialized")]
    RepositoryNotFoundOrInitialized,
    #[error("Repository already initialized")]
    RepositoryAlreadyInitialized,
    #[error("Not a directory")]
    NotADirectory,
    #[error("Yaml serialization error: `{0}`")]
    YamlSerializationError(#[from] serde_yml::Error),
    #[error("Module `{0}` already exists")]
    ModuleAlreadyExists(String),
    #[error("Module `{0}` is currently selected so it cannot be removed")]
    CannotRemoveSelectedModule(String),
    #[error("Module `{0}` does not exist")]
    ModuleDoesNotExists(String),
    #[error("Version `{0}` already exists")]
    VersionAlreadyExists(String),
    #[error("No current version selected in module `{0}`")]
    NoCurrentVersionInModule(String),
    #[error("No current module selected")]
    NoCurrentModule,
    #[error("Version `{0}` does not exist")]
    VersionDoesNotExists(String),
    #[error("Version `{0}` was not saved, so it can't be loaded")]
    VersionNotSaved(String),
    #[error("Path processing error: `{0}`")]
    PathProcessingError(#[from] StripPrefixError),
    #[error("Base decode error: `{0}`")]
    BaseDecodeError(#[from] DecodeError),
}
