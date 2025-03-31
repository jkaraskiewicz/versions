use std::{env, str::from_utf8};

use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use colored::Colorize;
use thiserror::Error;
use versions::{
    cli::{Cli, Command, ModuleCommand, VersionCommand},
    exists, init,
    local::{load_local_config, save_local_config, LocalConfig},
    open, Repository,
};

fn main() {
    let output = process().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });
    println!("{}", output);
}

fn process() -> Result<String, Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    let cli = Cli::parse();

    match cli.command {
        Command::Init => {
            if exists(&current_dir) {
                Err(VersionsCliError::CliError("Repository already initialized".to_string()).into())
            } else {
                init(&current_dir)?;
                Ok("Repository initialized successfully.".into())
            }
        }
        Command::Module { module_command } => process_module_command(&module_command),
        Command::Version {
            name,
            version_command,
        } => process_version_command(&name, &version_command),
        Command::Completions => {
            let mut buf = Vec::new();
            generate(current_shell(), &mut Cli::command(), "versions", &mut buf);
            Ok(from_utf8(buf.as_slice()).unwrap().to_string())
        }
        Command::List => {
            if !exists(&current_dir) {
                return Err(
                    VersionsCliError::CliError("Repository not initialized".to_string()).into(),
                );
            };
            let repository = open(&current_dir)?;
            list_entities(&repository, true)
        }
    }
}

fn current_shell() -> Shell {
    clap_complete::Shell::from_env().unwrap_or(Shell::Zsh)
}

fn process_module_command(
    module_command: &ModuleCommand,
) -> Result<String, Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;

    if !exists(&current_dir) {
        return Err(VersionsCliError::CliError("Repository not initialized".to_string()).into());
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
) -> Result<String, Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;

    if !exists(&current_dir) {
        return Err(VersionsCliError::CliError("Repository not initialized".to_string()).into());
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
        VersionCommand::List => {
            let version_name = repository.get_module(&module_name)?.current_version.name;
            let versions: Vec<String> = repository
                .get_module(&module_name)?
                .list_versions()
                .iter()
                .map(|version| {
                    if version.name == version_name {
                        format!("{}", version.name.bold().underline())
                    } else {
                        version.name.to_string()
                    }
                })
                .collect();
            Ok(versions.join("\n"))
        }
    }
}

fn list_entities(
    repository: &Repository,
    list_versions: bool,
) -> Result<String, Box<dyn std::error::Error>> {
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
                        if version.name == version_name {
                            format!("  {}", version.name.bold().underline())
                        } else {
                            format!("  {}", version.name)
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

#[derive(Error, Debug)]
pub enum VersionsCliError {
    #[error("{0}")]
    CliError(String),
}
