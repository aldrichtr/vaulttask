#[warn(missing_docs)]
// region imports
mod cli;
mod command;
mod config;
mod task;

use markdown_it::{parser::inline::{InlineRoot, Text}, plugins::cmark::block::heading::ATXHeading};
use markdown_it_front_matter::FrontMatter;
use serde_yaml;

use crate::{cli::Cli, config::Config, task::parser::Parser};
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
    let mut parser = Parser::new();

    // region: just for debug purposes
    println!("Debug set to {:?}", config.debug);

    println!("the path would be {:?}", config.vaults[0].to_glob());

    for entry in config.vaults[0]
        .get_files()
        .expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                println!("file: {:?}", path.display());
                let ast = parser.parse(&path);
                ast.walk(|node, _depth| {
                    if let Some(fm) = node.cast::<FrontMatter>() {
                        println!("The front matter content: {}", fm.content);
                    }
                    else if let Some(hd) = node.cast::<ATXHeading>() {
                        println!("Children are: {:#?}", node.children);
                        if let Some(h_title) = node.children[0].cast::<Text>() {
                            let h_title = h_title.content.as_str();
                            println!("This heading is level {} with title {}", hd.level, h_title);
                        }
                    } else {
                        println!("This node is {:?} at depth {}", node.name(), _depth);
                    }
                });
                // println!("md ast: {:?}", parser.parse(&path));
            }
            Err(e) => println!("error: {:?}", e),
        }
    }
    // endregion just for debug purposes

    let cli = Cli::new(config);
    cli.run();
}
