use std::path::PathBuf;

use clap::{Subcommand, Args};

#[derive(Debug, Args)]
pub struct VaultArgs {
    #[command[subcommand]]
    command: Option<VaultCommands>
}

#[derive(Debug, Subcommand)]
pub enum VaultCommands {
    Add {
        /// Add a vault to the configuration
        #[arg(required = true)]
        path: PathBuf,
        name: Option<String>,
        pattern: Option<String>
    },
    Remove {
        /// Remove a vault from the configuration
        #[arg(required = true)]
        path: PathBuf,
    },
}
