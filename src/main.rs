#[warn(missing_docs)]
// region imports
mod cli;
mod command;
mod config;
mod task;

use cli::Cli;
use config::Config;
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
    let config: Config = Config::new();

    // region: just for debug purposes
    println!( "Debug set to {:?}", config.debug);

    println!("the path would be {:?}", config.vaults[0].to_glob());

    for entry in config.vaults[0].get_files().expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => println!("file: {:?}", path.display()),
            Err(e) => println!("error: {:?}", e)
        }
    }
    // endregion just for debug purposes

    let cli = Cli::new(config);
    cli.run();

}
