use crate::common::{constants, errors::VersionsError};
use commons::utils::file_util::{read_file, write_file};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocalConfig {
    pub current_module: Option<String>,
}

pub fn save_local_config<P: AsRef<Path>>(
    repository_root_dir: P,
    config: &LocalConfig,
) -> Result<(), VersionsError> {
    let content = serde_yml::to_string(config)?;
    let file_path = repository_root_dir
        .as_ref()
        .join(constants::REPOSITORY_DIR)
        .join(constants::LOCAL_FILE);

    write_file(&file_path, &content)?;
    Ok(())
}

pub fn load_local_config<P: AsRef<Path>>(
    repository_root_dir: P,
) -> Result<LocalConfig, VersionsError> {
    let file_path = repository_root_dir
        .as_ref()
        .join(constants::REPOSITORY_DIR)
        .join(constants::LOCAL_FILE);

    let content = read_file(file_path)?;

    let config: LocalConfig = serde_yml::from_str(&content)?;
    Ok(config)
}
