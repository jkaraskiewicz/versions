use std::{env, str::from_utf8};

use clap::{CommandFactory, Parser};
use clap_complete::{generate, Generator, Shell};
use colored::Colorize;
use thiserror::Error;
use versions::{
    cli::{Cli, Command, ModuleCommand, VersionCommand},
    exists, init,
    local::{load_local_config, save_local_config, LocalConfig},
    open,
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
            let arg_matches = Cli::command().get_matches();
            if let Some(generator) = arg_matches.get_one::<Shell>("generator").copied() {
                let mut cmd = Cli::command();
                Ok(generate_completions(generator, &mut cmd))
            } else {
                Ok("Couldn't generate shell completions.".to_string())
            }
        }
    }
}

fn generate_completions<G: Generator>(generator: G, cmd: &mut clap::Command) -> String {
    let mut buf = Vec::new();
    generate(generator, cmd, cmd.get_name().to_string(), &mut buf);
    from_utf8(buf.as_slice()).unwrap().to_string()
}

fn process_module_command(
    module_command: &ModuleCommand,
) -> Result<String, Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;

    if !exists(&current_dir) {
        return Err(VersionsCliError::CliError("Repository not initialized".to_string()).into());
    };

    let repository = open(current_dir)?;

    match module_command {
        ModuleCommand::New { name, path } => {
            let _ = repository.add_module(name, path)?;
            Ok(format!("Module {} added.", name.bold()))
        }
        ModuleCommand::Remove { name } => {
            let module = repository.get_module(name)?;
            repository.remove_module(&module)?;
            Ok(format!("Module {} removed.", name.bold()))
        }
        ModuleCommand::List => {
            let modules = repository.list_modules()?;
            let lines: Vec<String> = modules
                .iter()
                .map(|module| format!("{} {}", module.name, module.directory.dimmed()))
                .collect();
            Ok(lines.join("\n"))
        }
        ModuleCommand::Select { name } => {
            let module = repository.get_module(name)?;
            save_local_config(
                repository.root_path,
                &LocalConfig {
                    current_module: Some(module.name),
                },
            )?;
            Ok(format!("Module {} selected.", name.bold()))
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
        None => load_local_config(&current_dir)?.current_module.unwrap(),
    };

    match version_command {
        VersionCommand::New { name } => {
            let _ = repository.get_module(&module_name)?.add_version(name)?;
            Ok(format!("Version {} added.", name.bold()))
        }
        VersionCommand::Remove { name } => {
            repository.get_module(&module_name)?.remove_version(name)?;
            Ok(format!("Version {} removed.", name.bold()))
        }
        VersionCommand::Select { name } => {
            let _ = repository.get_module(&module_name)?.switch_version(name)?;
            Ok(format!("Version {} selected.", name.bold()))
        }
        VersionCommand::Current => {
            let version_name = repository.get_module(&module_name)?.current_version.name;
            Ok(version_name)
        }
        VersionCommand::List => {
            let versions: Vec<String> = repository
                .get_module(&module_name)?
                .list_versions()?
                .iter()
                .map(|version| version.name.to_string())
                .collect();
            Ok(versions.join("\n"))
        }
    }
}

#[derive(Error, Debug)]
pub enum VersionsCliError {
    #[error("Cli error: `{0}`")]
    CliError(String),
}
