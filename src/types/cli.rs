use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueHint};
#[derive(Parser, Debug)]
#[command(author, version, about = "Simple version control system")]
pub struct Cli {
    /// Command
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug, Clone, PartialEq)]
#[clap(rename_all = "lower_case")]
pub enum Command {
    /// Initialize repository
    Init,
    /// Module commands
    Module {
        #[clap(subcommand)]
        module_command: ModuleCommand,
    },
    /// Version commands
    Version {
        /// Name of the module
        #[arg(default_value = None)]
        name: Option<String>,
        #[clap(subcommand)]
        version_command: VersionCommand,
    },
    /// Show repository state (modules, versions)
    Show,
    /// Generate shell completions
    Completions,
}

#[derive(Subcommand, Debug, Clone, PartialEq)]
#[clap(rename_all = "lower_case")]
pub enum ModuleCommand {
    /// Create module
    Add {
        /// Name of the module
        #[arg()]
        name: String,
        /// Path to the directory
        #[arg(default_value = None, value_hint = ValueHint::DirPath)]
        path: Option<PathBuf>,
    },
    /// Remove module
    Remove {
        /// Name of the module
        #[arg()]
        name: String,
    },
    /// Select module
    Select {
        /// Name of the module
        #[arg()]
        name: String,
    },
    /// Deselect current module
    Deselect,
    /// List modules
    List,
    /// Current module
    Current,
}

#[derive(Subcommand, Debug, Clone, PartialEq)]
#[clap(rename_all = "lower_case")]
pub enum VersionCommand {
    /// Create new version
    Add {
        /// Name of the version
        #[arg()]
        name: String,
    },
    /// Remove version
    Remove {
        /// Name of the version
        #[arg()]
        name: String,
    },
    /// Select version
    Select {
        /// Name of the version
        #[arg()]
        name: String,
    },
    /// Deselect version
    Deselect,
    /// List versions
    List,
    /// Current version
    Current,
    /// Workspace status
    Status,
    /// Save current version
    Save,
    /// Load last snapshot of the current version
    Load,
}
