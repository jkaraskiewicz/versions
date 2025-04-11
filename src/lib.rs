use colored::Colorize;
pub use common::errors::VersionsError;
pub use common::version_util::get_version_object_file_path;
use commons::utils::datetime_util::formatted_systemtime;
use handlers::repository_handler;
use std::env::{self};
use std::path::{Path, PathBuf};
pub use types::cli;
pub use types::module::Module;
pub use types::repository::Repository;
pub use types::version::Version;
use types::{
    cli::{ModuleCommand, VersionCommand},
    modules_config::read_modules_config,
};

mod common;
mod handlers;
mod types;

pub fn init<P: AsRef<Path>>(path: P) -> Result<Repository, VersionsError> {
    repository_handler::init(path)
}

pub fn open<P: AsRef<Path>>(path: P, look_up: bool) -> Result<Repository, VersionsError> {
    repository_handler::open(path, look_up)
}

pub fn exists<P: AsRef<Path>>(path: P, look_up: bool) -> Option<PathBuf> {
    repository_handler::exists(path, look_up)
}

pub fn commands() -> VersionsCliCommand {
    VersionsCliCommand {}
}

// CLI exports

pub struct VersionsCliCommand {}

impl VersionsCliCommand {
    pub fn current_module(&self) -> Result<Option<String>, VersionsError> {
        let current_dir = env::current_dir()?;
        let repository = open(&current_dir, true)?;

        let selected_module_name = read_modules_config(&repository)?.current_module;

        Ok(selected_module_name.map(|el| el.name))
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

        if exists(&current_dir, false).is_some() {
            Err(VersionsError::RepositoryAlreadyInitialized)
        } else {
            init(&current_dir)?;
            Ok("Repository initialized successfully.".into())
        }
    }

    pub fn show(&self) -> Result<String, VersionsError> {
        let current_dir = env::current_dir()?;
        let repository = open(&current_dir, true)?;
        let repository_str = format!(
            "Repository root: {}",
            repository.root_path.to_str().unwrap().dimmed()
        );
        let entities = list_entities(&repository, true)?;
        if !entities.is_empty() {
            Ok(format!("{}\n{}", repository_str, entities))
        } else {
            Ok(repository_str.to_string())
        }
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
    let repository = open(&current_dir, true)?;

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
            let selected_module_name = current_module_name(&repository)?;
            if selected_module_name.is_empty() {
                Ok("<No selected module>".to_string())
            } else {
                Ok(selected_module_name.bold().underline().to_string())
            }
        }
        ModuleCommand::Select { name } => {
            let module = repository.get_module(name)?;
            repository.select_module(&module)?;
            Ok(format!("Module {} selected.", name.bold().underline()))
        }
    }
}

fn process_version_command(
    module_name: &Option<String>,
    version_command: &VersionCommand,
) -> Result<String, VersionsError> {
    let current_dir = env::current_dir()?;
    let repository = open(&current_dir, true)?;

    let module_name = match module_name {
        Some(module_name) => module_name.to_string(),
        None => current_module_name(&repository)?,
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
            let _ = repository.get_module(&module_name)?.select_version(name)?;
            Ok(format!("Version {} selected.", name.bold().underline()))
        }
        VersionCommand::Current => {
            let version_name = repository
                .get_module(&module_name)?
                .force_current_version()?
                .name;
            Ok(format!("{}", version_name.bold().underline()))
        }
        VersionCommand::Status => {
            let status = repository
                .get_module(&module_name)?
                .force_current_version()?
                .status()?;
            Ok(status.unwrap_or("Workspace clean.".to_string()))
        }
        VersionCommand::Save => {
            let current_version = repository
                .get_module(&module_name)?
                .force_current_version()?;
            current_version.save()?;
            Ok(format!(
                "Version {} saved.",
                current_version.name.bold().underline()
            ))
        }
        VersionCommand::Load => {
            let current_version = repository
                .get_module(&module_name)?
                .force_current_version()?;
            current_version.load()?;
            Ok(format!(
                "Last snapshot of version {} loaded.",
                current_version.name.bold().underline()
            ))
        }
        VersionCommand::List => {
            let version_name = repository
                .get_module(&module_name)?
                .current_version
                .map(|v| v.name);
            let versions: Vec<String> = repository
                .get_module(&module_name)?
                .list_versions()
                .iter()
                .map(|version| {
                    let path = get_version_object_file_path(version);
                    let modified = path.metadata().unwrap().modified().unwrap();
                    let time = formatted_systemtime(&modified);
                    let version_name = version_name.to_owned().unwrap_or_default();
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
    let selected_module_name = current_module_name(repository)?;
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
                let version_name = module.current_version.to_owned().map(|v| v.name);
                let versions: Vec<String> = module
                    .list_versions()
                    .iter()
                    .map(|version| {
                        let path = get_version_object_file_path(version);
                        let modified = path.metadata().unwrap().modified().unwrap();
                        let time = formatted_systemtime(&modified);
                        let version_name = version_name.to_owned().unwrap_or_default();
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

fn current_module_name(repository: &Repository) -> Result<String, VersionsError> {
    let selected_module_name = read_modules_config(repository)?
        .current_module
        .map(|el| el.name)
        .unwrap_or_default();
    Ok(selected_module_name)
}
