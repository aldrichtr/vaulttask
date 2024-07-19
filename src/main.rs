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

    let config: Config = Config::new();
    let cli = Cli::new(config);
    cli.run();



}
