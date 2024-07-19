use clap::Parser;

use crate::{command::Commands, config::Config};

#[derive(Debug)]
pub struct Cli {
    name: String,
    config: Config,
}

#[derive(Debug, Parser)]
#[command(name = env!("CARGO_BIN_NAME"))]
#[command(about = "A task management system that uses a vault of markdown files as its data")]
struct CommandLine {
    #[command(subcommand)]
    command: Commands,

}

impl Cli {
    pub fn new(conf: Config) -> Self {
        Self {
            name: String::from(env!("CARGO_BIN_NAME")),
            config: conf
        }
    }
    pub fn run(&self) {
        let args = CommandLine::parse();
        print!("The args are {:?}", args)
    }
}
