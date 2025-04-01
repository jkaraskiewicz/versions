use std::fs::{self, create_dir_all};

use super::meta::ModulePtr;
use crate::common::{
    constants,
    diff_util::get_version_files_diff,
    errors::VersionsError,
    flate_util,
    stream_util::{self, StreamEntriesSet},
    version_util::get_file_name,
};
use commons::utils::hash_util::get_string_hash;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Version {
    pub name: String,
    #[serde(skip)]
    pub module: ModulePtr,
}

impl Version {
    pub fn save(&self) -> Result<(), VersionsError> {
        let mut file_name = get_file_name(self);
        file_name = get_string_hash(&file_name);

        let dir_path = self
            .module
            .repository_path
            .join(&self.module.module_dir)
            .to_path_buf();

        let output_file_path = self
            .module
            .repository_path
            .join(constants::REPOSITORY_DIR)
            .join(constants::OBJECTS_DIR)
            .join(file_name)
            .to_path_buf();

        flate_util::flate_directory(dir_path, output_file_path)?;

        Ok(())
    }

    pub fn load(&self) -> Result<(), VersionsError> {
        let mut file_name = get_file_name(self);
        file_name = get_string_hash(&file_name);

        let input_file_path = self
            .module
            .repository_path
            .join(constants::REPOSITORY_DIR)
            .join(constants::OBJECTS_DIR)
            .join(file_name)
            .to_path_buf();

        if !input_file_path.exists() {
            return Err(VersionsError::VersionNotSaved(self.name.to_string()));
        }

        let output_dir_path = self
            .module
            .repository_path
            .join(&self.module.module_dir)
            .to_path_buf();

        fs::remove_dir_all(&output_dir_path).unwrap_or_default();
        create_dir_all(&output_dir_path)?;
        flate_util::deflate_directory(input_file_path, output_dir_path)?;

        Ok(())
    }

    pub fn status(&self) -> Result<Option<String>, VersionsError> {
        let mut file_name = get_file_name(self);
        file_name = get_string_hash(&file_name);

        let input_file_path = self
            .module
            .repository_path
            .join(constants::REPOSITORY_DIR)
            .join(constants::OBJECTS_DIR)
            .join(file_name)
            .to_path_buf();

        let dir_path = self
            .module
            .repository_path
            .join(&self.module.module_dir)
            .to_path_buf();

        let current_content = stream_util::stream_dir(&dir_path)?;
        let current_entries_set: StreamEntriesSet = toml::from_str(&current_content)?;
        let saved_entries_set = if input_file_path.exists() {
            let saved_content = flate_util::deflate_to_string(&input_file_path)?;
            toml::from_str(&saved_content)?
        } else {
            StreamEntriesSet {
                entries: Vec::new(),
            }
        };
        get_version_files_diff(&saved_entries_set, &current_entries_set, &self.module)
    }

    pub fn remove(&self) -> Result<(), VersionsError> {
        let mut file_name = get_file_name(&self);
        file_name = get_string_hash(&file_name);

        let input_file_path = self
            .module
            .repository_path
            .join(constants::REPOSITORY_DIR)
            .join(constants::OBJECTS_DIR)
            .join(file_name)
            .to_path_buf();

        if !input_file_path.exists() {
            return Err(VersionsError::VersionNotSaved(self.name.to_string()));
        }

        fs::remove_file(input_file_path)?;
        Ok(())
    }
}
