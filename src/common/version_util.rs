use super::constants;
use crate::types::version::Version;
use commons::utils::hash_util::get_string_hash;
use std::path::PathBuf;

pub fn get_version_object_file_path(version: &Version) -> PathBuf {
    let file_name = get_string_hash(get_file_name(version).as_str());
    version
        .module
        .repository_path
        .join(constants::REPOSITORY_DIR)
        .join(constants::OBJECTS_DIR)
        .join(file_name)
}

pub fn get_file_name(version: &Version) -> String {
    format!("{}#{}", version.module.module_dir, version.name)
}
