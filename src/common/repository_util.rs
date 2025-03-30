use std::path::Path;

use crate::types::repository::Repository;

pub fn from_path<P: AsRef<Path>>(path: P) -> Repository {
    Repository {
        root_path: path.as_ref().to_path_buf(),
    }
}
