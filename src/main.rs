use std::str::from_utf8;

use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use versions::{
    cli::{Cli, Command},
    VersionsCli, VersionsError,
};

fn main() {
    let output = process().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });
    println!("{}", output);
}

fn process() -> Result<String, VersionsError> {
    let cli = Cli::parse();
    let version_cli = VersionsCli::new();

    match cli.command {
        Command::Init => version_cli.init(),
        Command::Module { module_command } => version_cli.module(&module_command),
        Command::Version {
            name,
            version_command,
        } => version_cli.version(&name, &version_command),
        Command::Completions => {
            let mut buf = Vec::new();
            generate(current_shell(), &mut Cli::command(), "versions", &mut buf);
            Ok(from_utf8(buf.as_slice()).unwrap().to_string())
        }
        Command::State => version_cli.state(),
    }
}

fn current_shell() -> Shell {
    clap_complete::Shell::from_env().unwrap_or(Shell::Zsh)
}
