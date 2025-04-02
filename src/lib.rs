use colored::Colorize;
pub use common::errors::VersionsError;
pub use common::version_util::get_version_object_file_path;
use commons::utils::datetime_util::formatted_systemtime;
use handlers::repository_handler;
use std::env;
use std::path::Path;
pub use types::cli;
use types::cli::{ModuleCommand, VersionCommand};
pub use types::local;
use types::local::{load_local_config, save_local_config, LocalConfig};
pub use types::repository::Repository;

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

pub fn commands() -> VersionsCliCommand {
    VersionsCliCommand {}
}

// CLI exports

pub struct VersionsCliCommand {}

impl VersionsCliCommand {
    pub fn current_module(&self) -> Result<Option<String>, VersionsError> {
        let current_dir = env::current_dir()?;

        if !exists(&current_dir) {
            return Err(VersionsError::RepositoryNotInitialized);
        };

        let repository = open(&current_dir)?;

        let selected_module_name = load_local_config(&repository.root_path)
            .unwrap_or_default()
            .current_module;

        Ok(selected_module_name)
    }
}

pub struct VersionsCli {}

impl Default for VersionsCli {
    fn default() -> Self {
        Self::new()
    }
}

impl VersionsCli {
    pub fn new() -> Self {
        VersionsCli {}
    }

    pub fn init(&self) -> Result<String, VersionsError> {
        let current_dir = env::current_dir()?;

        if exists(&current_dir) {
            Err(VersionsError::RepositoryAlreadyInitialized)
        } else {
            init(&current_dir)?;
            Ok("Repository initialized successfully.".into())
        }
    }

    pub fn list(&self) -> Result<String, VersionsError> {
        let current_dir = env::current_dir()?;
        if !exists(&current_dir) {
            return Err(VersionsError::RepositoryNotInitialized);
        };
        let repository = open(&current_dir)?;
        list_entities(&repository, true)
    }

    pub fn module(&self, module_command: &ModuleCommand) -> Result<String, VersionsError> {
        process_module_command(module_command)
    }

    pub fn version(
        &self,
        module_name: &Option<String>,
        version_command: &VersionCommand,
    ) -> Result<String, VersionsError> {
        process_version_command(module_name, version_command)
    }
}

fn process_module_command(module_command: &ModuleCommand) -> Result<String, VersionsError> {
    let current_dir = env::current_dir()?;

    if !exists(&current_dir) {
        return Err(VersionsError::RepositoryNotInitialized);
    };

    let repository = open(&current_dir)?;

    match module_command {
        ModuleCommand::Add { name, path } => {
            let path = path.to_owned().unwrap_or(current_dir.join(name));
            let _ = repository.add_module(name, path)?;
            Ok(format!("Module {} added.", name.bold().underline()))
        }
        ModuleCommand::Remove { name } => {
            let module = repository.get_module(name)?;
            repository.remove_module(&module)?;
            Ok(format!("Module {} removed.", name.bold().underline()))
        }
        ModuleCommand::List => list_entities(&repository, false),
        ModuleCommand::Current => {
            let selected_module_name = load_local_config(&repository.root_path)
                .unwrap_or_default()
                .current_module
                .unwrap_or_default();
            if selected_module_name.is_empty() {
                Ok("<No selected module>".to_string())
            } else {
                Ok(selected_module_name.bold().underline().to_string())
            }
        }
        ModuleCommand::Select { name } => {
            let module = repository.get_module(name)?;
            save_local_config(
                repository.root_path,
                &LocalConfig {
                    current_module: Some(module.name),
                },
            )?;
            Ok(format!("Module {} selected.", name.bold().underline()))
        }
    }
}

fn process_version_command(
    module_name: &Option<String>,
    version_command: &VersionCommand,
) -> Result<String, VersionsError> {
    let current_dir = env::current_dir()?;

    if !exists(&current_dir) {
        return Err(VersionsError::RepositoryNotInitialized);
    };

    let repository = open(&current_dir)?;
    let module_name = match module_name {
        Some(module_name) => module_name.to_string(),
        None => load_local_config(&current_dir)
            .unwrap_or_default()
            .current_module
            .unwrap_or_default(),
    };

    match version_command {
        VersionCommand::Add { name } => {
            let _ = repository.get_module(&module_name)?.add_version(name)?;
            Ok(format!("Version {} added.", name.bold().underline()))
        }
        VersionCommand::Remove { name } => {
            repository.get_module(&module_name)?.remove_version(name)?;
            Ok(format!("Version {} removed.", name.bold().underline()))
        }
        VersionCommand::Select { name } => {
            let _ = repository.get_module(&module_name)?.switch_version(name)?;
            Ok(format!("Version {} selected.", name.bold().underline()))
        }
        VersionCommand::Current => {
            let version_name = repository.get_module(&module_name)?.current_version.name;
            Ok(format!("{}", version_name.bold().underline()))
        }
        VersionCommand::Status => {
            let status = repository
                .get_module(&module_name)?
                .current_version
                .status()?;
            Ok(status.unwrap_or("Workspace clean.".to_string()))
        }
        VersionCommand::Save => {
            let current_version = repository.get_module(&module_name)?.current_version;
            current_version.save()?;
            Ok(format!(
                "Version {} saved.",
                current_version.name.bold().underline()
            ))
        }
        VersionCommand::Load => {
            let current_version = repository.get_module(&module_name)?.current_version;
            current_version.load()?;
            Ok(format!(
                "Last snapshot of version {} loaded.",
                current_version.name.bold().underline()
            ))
        }
        VersionCommand::List => {
            let version_name = repository.get_module(&module_name)?.current_version.name;
            let versions: Vec<String> = repository
                .get_module(&module_name)?
                .list_versions()
                .iter()
                .map(|version| {
                    let path = get_version_object_file_path(version);
                    let modified = path.metadata().unwrap().modified().unwrap();
                    let time = formatted_systemtime(&modified);
                    if version.name == version_name {
                        format!("{} ({})", version.name.bold().underline(), time.dimmed())
                    } else {
                        format!("{} {}", version.name, time.dimmed())
                    }
                })
                .collect();
            Ok(versions.join("\n"))
        }
    }
}

fn list_entities(repository: &Repository, list_versions: bool) -> Result<String, VersionsError> {
    let selected_module_name = load_local_config(&repository.root_path)
        .unwrap_or_default()
        .current_module
        .unwrap_or_default();
    let modules = repository.list_modules()?;
    let max_name_length = modules
        .iter()
        .max_by(|x, y| x.name.len().cmp(&y.name.len()))
        .map(|m| m.name.len())
        .unwrap_or_default();
    let lines: Vec<String> = modules
        .iter()
        .flat_map(|module| {
            let mut module_lines: Vec<String> = vec![];
            let module_str = if module.name == selected_module_name {
                module.name.bold().underline()
            } else {
                module.name.normal()
            };
            module_lines.push(format!(
                "{}{:length$}{}",
                module_str,
                " ",
                module.directory.dimmed(),
                length = max_name_length - module_str.len()
            ));
            if list_versions {
                let version_name = module.current_version.name.to_string();
                let versions: Vec<String> = module
                    .list_versions()
                    .iter()
                    .map(|version| {
                        let path = get_version_object_file_path(version);
                        let modified = path.metadata().unwrap().modified().unwrap();
                        let time = formatted_systemtime(&modified);
                        if version.name == version_name {
                            format!("  {} ({})", version.name.bold().underline(), time.dimmed())
                        } else {
                            format!("  {} ({})", version.name, time.dimmed())
                        }
                    })
                    .collect();
                module_lines.extend(versions);
            };
            module_lines
        })
        .collect();
    Ok(lines.join("\n"))
}
