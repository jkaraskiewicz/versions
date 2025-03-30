use std::fs::{self, create_dir_all};

use super::meta::ModulePtr;
use crate::common::{constants, errors::VersionsError, flate_util, stream_util};
use commons::utils::hash_util::get_string_hash;
use diffy::{create_patch, PatchFormatter};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Version {
    pub name: String,
    #[serde(skip)]
    pub module: ModulePtr,
}

impl Version {
    pub fn save(&self) -> Result<(), VersionsError> {
        let mut file_name = self.get_file_name();
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
        let mut file_name = self.get_file_name();
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
        let mut file_name = self.get_file_name();
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
        let saved_content = if input_file_path.exists() {
            flate_util::deflate_to_string(&input_file_path)?
        } else {
            "".to_string()
        };

        let patch = create_patch(&saved_content, &current_content);
        if patch.hunks().is_empty() {
            Ok(None)
        } else {
            let formatter = PatchFormatter::new().with_color();
            Ok(Some(format!("{}", formatter.fmt_patch(&patch))))
        }
    }

    pub fn remove(&self) -> Result<(), VersionsError> {
        let mut file_name = self.get_file_name();
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

    fn get_file_name(&self) -> String {
        format!("{}#{}", self.module.module_dir, self.name)
    }
}
