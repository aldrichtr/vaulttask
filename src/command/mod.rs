pub mod vault;

use clap::Subcommand;

use crate::command::vault::VaultArgs;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Vault(VaultArgs)
}
