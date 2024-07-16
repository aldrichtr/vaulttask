pub mod config;

// #region imports
use config::Config;

// #endregion imports


fn main() {
    // get commandline arguments
    // pass them to the config


    let config: Config = Config::new();

    println!( "Debug set to {:?}", config.debug);

    println!("the path would be {:?}", config.vaults[0].to_glob());

    for entry in config.vaults[0].get_files().expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => println!("file: {:?}", path.display()),
            Err(e) => println!("error: {:?}", e)
        }
    }
}
