use std::path::{Path, PathBuf};

use crate::types::meta::ModulePtr;

use super::{
    errors::VersionsError,
    stream_util::{StreamEntriesSet, StreamEntry, StreamEntryType},
};
use colored::Colorize;
use diffy::{create_patch, PatchFormatter};

pub fn get_version_files_diff(
    original: &StreamEntriesSet,
    modified: &StreamEntriesSet,
    module: &ModulePtr,
) -> Result<Option<String>, VersionsError> {
    let mut result_diff: Vec<String> = Vec::new();

    let original_entries = original.entries.to_vec();
    let modified_entries = modified.entries.to_vec();

    for entry in &modified_entries {
        let original_entry_equivalent = original_entries
            .iter()
            .find(|el| el.relative_path == entry.relative_path)
            .map(|el| el.to_owned());
        let diff = get_stream_entry_diff(
            original_entry_equivalent.to_owned(),
            Some(entry.to_owned()),
            module,
        )?;
        result_diff.extend(diff);
    }

    for entry in &original_entries {
        let modified_entry_equivalent = modified_entries
            .iter()
            .find(|el| el.relative_path == entry.relative_path);
        if let Some(_) = modified_entry_equivalent {
            continue;
        } else {
            let diff = get_stream_entry_diff(Some(entry.to_owned()), None, module)?;
            result_diff.extend(diff);
        }
    }
    if result_diff.is_empty() {
        Ok(None)
    } else {
        Ok(Some(result_diff.join("\n")))
    }
}

fn get_stream_entry_diff(
    original: Option<StreamEntry>,
    modified: Option<StreamEntry>,
    module: &ModulePtr,
) -> Result<Vec<String>, VersionsError> {
    let mut result: Vec<String> = Vec::new();
    let formatter = PatchFormatter::new().with_color();

    if let Some(original) = original {
        match modified {
            Some(modified) => {
                let path = PathBuf::new()
                    .join(&module.module_dir)
                    .join(&original.relative_path);
                if should_print_header(&original.relative_path, module)? {
                    result.push(
                        format!("{}", path.to_str().unwrap())
                            .dimmed()
                            .underline()
                            .to_string(),
                    );
                }
                if original.entry_type == StreamEntryType::File {
                    let original_content = original.content.unwrap_or_default();
                    let modified_content = modified.content.unwrap_or_default();
                    let patch = create_patch(&original_content, &modified_content);
                    if !patch.hunks().is_empty() {
                        result.push(format!("{}", formatter.fmt_patch(&patch)));
                    };
                }
            }
            None => {
                let path = PathBuf::new()
                    .join(&module.module_dir)
                    .join(&original.relative_path);
                if should_print_header(&original.relative_path, module)? {
                    result.push(
                        format!("- {}", path.to_str().unwrap())
                            .dimmed()
                            .underline()
                            .to_string(),
                    );
                }
                if original.entry_type == StreamEntryType::File {
                    let original_content = original.content.unwrap_or_default();
                    let patch = create_patch(&original_content, "");
                    if !patch.hunks().is_empty() {
                        result.push(format!("{}", formatter.fmt_patch(&patch)));
                    } else {
                        result.push("<Empty file>".italic().to_string());
                    };
                }
            }
        }
    } else if let Some(modified) = modified {
        let path = PathBuf::new()
            .join(&module.module_dir)
            .join(&modified.relative_path);
        if should_print_header(&modified.relative_path, module)? {
            result.push(
                format!("+ {}", path.to_str().unwrap())
                    .dimmed()
                    .underline()
                    .to_string(),
            );
        }
        if modified.entry_type == StreamEntryType::File {
            let modified_content = modified.content.unwrap_or_default();
            let patch = create_patch("", &modified_content);
            if !patch.hunks().is_empty() {
                result.push(format!("{}", formatter.fmt_patch(&patch)));
            } else {
                result.push("<Empty file>".italic().to_string());
            };
        }
    };

    Ok(result)
}

fn should_print_header(relative_path: &Path, module: &ModulePtr) -> Result<bool, VersionsError> {
    let full_path = PathBuf::from(&module.repository_path)
        .join(&module.module_dir)
        .join(relative_path);
    if !full_path.is_dir() {
        return Ok(true);
    }
    Ok(full_path.read_dir()?.next().is_none())
}
