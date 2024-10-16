// region: Imports
mod task;
mod vault;

//- std lib
use std::{default::Default, path::PathBuf, vec::Vec};

//- crates
use clap::Parser;
use etcetera::{choose_base_strategy, BaseStrategy};
use serde_derive::{Deserialize, Serialize};

//- local
use crate::{
    command::{Commands, vault::VaultArgs, vault::VaultCommands},
    config::{task::TaskConfig, vault::VaultConfig},
};
// endregion Imports

const APP_NAME : &str = "vtask";
const CONFIG_EXT : &str = ".config.yml";

// region: Task

/// # CommandLine
///
/// Represents the command line of the program.  The actual command line arguments and options.
/// Implements the clap::Parser
#[derive(Debug, Parser)]
#[command(name = env!("CARGO_BIN_NAME"))]
#[command(about = "A task management system that uses a vault of markdown files as its data")]
struct CommandLine {
    #[command(subcommand)]
    command : Commands,
}

// region: Config
#[derive(Debug, Serialize, Deserialize)]
#[allow(unused)]
pub struct Config {
    #[serde(default)]
    pub debug : bool,
    #[serde(default)]
    pub vaults : Vec<VaultConfig>,
    #[serde(default)]
    pub task : TaskConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            debug : false,
            vaults : Vec::<VaultConfig>::default(),
            task : TaskConfig::default(),
        }
    }
}

impl Config {
    pub fn new(path : Option<PathBuf>) -> Self {
        let path = path.unwrap_or(Config::get_config_file());
        dbg!("loading config file {}", path.display());
        let conf : Config = match confy::load_path(path) {
            Ok(cfg) => cfg,
            Err(err) => {
                panic!("Error loading config: {:?}", err);
            }
        };
        conf
    }

    pub fn get_config_dir() -> PathBuf {
        let strategy = choose_base_strategy().unwrap();
        // look in the default `<config directory for this os>/APP_NAME`
        strategy.config_dir().join(APP_NAME)
    }

    pub fn get_config_file() -> PathBuf {
        let dir = &Config::get_config_dir();
        dbg!("config file is {:#?}", dir);
        let file = APP_NAME.to_string() + CONFIG_EXT;
        dir.join(file)
    }

    pub fn parse_args(&self) {
        let args : CommandLine = CommandLine::parse();
        dbg!("Args parsed to {:#?}", args);
        // TODO: modify the config based on the commandline args
        match args.command {
            Commands::Vault(vault) => {
                let vault_cmd = vault.command.unwrap_or(VaultCommands::Add { path: (), name: (), pattern: () })
                match vault_args {
                    VaultCommands::Add {
                        path,
                        name,
                        pattern } => {
                        todo!("Implement adding a vaultConfig")
                    },
                    VaultCommands::Remove { path } => {
                        todo!("Implement removing a vaultConfig")
                    }
                }
            }
        }
    }
}

// endregion Config

// region: Tests
// endregion Tests
