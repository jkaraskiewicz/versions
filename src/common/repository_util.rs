use std::path::Path;

use crate::types::repository::Repository;

use super::constants;

pub fn from_path<P: AsRef<Path>>(path: P) -> Repository {
    Repository {
        root_path: path.as_ref().to_path_buf(),
    }
}

pub fn hosts_repository<P: AsRef<Path>>(path: P) -> bool {
    let modules_file_path = path
        .as_ref()
        .join(constants::REPOSITORY_DIR)
        .join(constants::MODULES_FILE);
    let objects_dir_path = path
        .as_ref()
        .join(constants::REPOSITORY_DIR)
        .join(constants::OBJECTS_DIR);
    modules_file_path.exists() && objects_dir_path.exists()
}
