use super::errors::VersionsError;
use base64::{prelude::BASE64_STANDARD, Engine};
use commons::utils::file_util::{read_binary_file, write_binary_file};
use serde::{Deserialize, Serialize};
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn stream_dir(dir_path: &Path) -> Result<String, VersionsError> {
    let mut stream_entries: Vec<StreamEntry> = vec![];
    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        let relative_path = entry.path().strip_prefix(dir_path)?;
        let content = if entry.path().is_dir() {
            None
        } else {
            let file_content = read_binary_file(entry.path())?;
            let based_file_content = BASE64_STANDARD.encode(file_content);
            Some(based_file_content)
        };
        let stream_entry = StreamEntry::create(
            StreamEntryType::for_path(entry.path()),
            relative_path,
            content,
        );
        stream_entries.push(stream_entry);
    }
    stream_entries.sort_by(|a, b| a.entry_type.cmp(&b.entry_type));
    let stream_entries_set = StreamEntriesSet {
        entries: stream_entries,
    };
    let output = toml::to_string(&stream_entries_set)?;
    Ok(output)
}

pub fn destream_dir(content: &str, target_dir_path: &Path) -> Result<(), VersionsError> {
    let stream_entries_set: StreamEntriesSet = toml::from_str(content)?;
    for entry in stream_entries_set.entries {
        let new_path = target_dir_path.join(&entry.relative_path);
        match entry.entry_type {
            StreamEntryType::Directory => {
                create_dir_all(new_path)?;
            }
            StreamEntryType::File => {
                let content = entry.content.unwrap();
                let unbased_content = BASE64_STANDARD.decode(&content)?;
                write_binary_file(new_path, &unbased_content)?;
            }
        }
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamEntriesSet {
    pub entries: Vec<StreamEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StreamEntry {
    pub entry_type: StreamEntryType,
    pub relative_path: PathBuf,
    pub content: Option<String>,
}

impl StreamEntry {
    fn create(entry_type: StreamEntryType, relative_path: &Path, content: Option<String>) -> Self {
        StreamEntry {
            entry_type,
            relative_path: relative_path.to_path_buf(),
            content,
        }
    }
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Ord, PartialOrd, Eq)]
pub enum StreamEntryType {
    File,
    Directory,
}

impl StreamEntryType {
    fn for_path(path: &Path) -> Self {
        if path.is_dir() {
            Self::Directory
        } else {
            Self::File
        }
    }
}
