//! # vtask
//!
//! `vtask` is a command line task manager that reads data from a set of markdown files in a `vault`.
//!
//! Several Knowlege Management Systems use the concept of a vault to denote a collection of files and folders, such
//! as obsidian, dendron, and others.
//! `vtask` allows you to manage tasks in the same format (and potentially the same files) as your notes.

// region: imports
mod command;
mod config;
mod data;

use crate::{config::Config, data::parser::Parser};
// endregion imports

fn main() {
    // 1. determine if we have ever run before.
    //    - we never have
    //      1. Ask where the vault(s) are
    //      2. Create a new configuration file
    //    - we have
    //      1. Load the configuration file
    //      2. Augment the config with command line arguments
    // 2. get the subcommand

    let config = &Config::new(None);
    config.parse_args();
    let mut parser = Parser::new();

    // region: just for debug purposes
    println!("Debug set to {:?}", config.debug);

    // This is fine for testing but it doesn't actually belong in main
    // ? should this be the get_tasks function?
    let vaults = &config.vaults;
    for vault in vaults {
        for entry in vault.get_files().expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    println!("file: {:?}", path.display());
                    let task = parser.parse(&path);
                    println!("Task is {:#?}", task);
                }
                Err(e) => {
                    println!("Could not get files from vault {}", e.error())
                }
            }
        }
    }
    // endregion just for debug purposes
}
