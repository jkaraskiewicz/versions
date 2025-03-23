use common::errors::VersionsError;
use handlers::repository_handler;
use std::path::PathBuf;
use types::repository::Repository;

mod common;
mod handlers;
mod types;

pub fn init(path: &PathBuf) -> Result<Repository, VersionsError> {
    repository_handler::init(path)
}

pub fn open(path: &PathBuf) -> Result<Repository, VersionsError> {
    repository_handler::open(path)
}

pub fn exists(path: &PathBuf) -> bool {
    repository_handler::exists(path)
}
