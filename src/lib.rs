use common::errors::VersionsError;
use handlers::repository_handler;
use std::path::Path;
pub use types::cli;
pub use types::local;
use types::repository::Repository;

mod common;
mod handlers;
mod types;

pub fn init<P: AsRef<Path>>(path: P) -> Result<Repository, VersionsError> {
    repository_handler::init(path)
}

pub fn open<P: AsRef<Path>>(path: P) -> Result<Repository, VersionsError> {
    repository_handler::open(path)
}

pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    repository_handler::exists(path)
}
