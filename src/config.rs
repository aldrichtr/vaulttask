// region Imports
use std::{
    path::PathBuf,
    vec::Vec,
    default::Default
};

use etcetera::{choose_base_strategy, BaseStrategy};
use serde_derive::{Deserialize, Serialize};
use glob::{glob_with, MatchOptions, Paths, PatternError};
use shellexpand;
use clap::{Parser, Subcommand};
use confy::ConfyError;

use crate::task::id::IdType;

// endregion Imports

const APP_NAME: &str = "vtask";
const CONFIG_EXT: &str = ".config";


// region: Vault
#[derive(Debug, Serialize, Deserialize)]
#[allow(unused)]
pub struct Vault {
    name: String,
    path: PathBuf,
    pattern: String,
    case_sensitive: bool,
}

impl Default for Vault {
    fn default() -> Self {
        Self {
            name: String::from(""),
            path: PathBuf::new(),
            pattern: String::from("*.md"),
            // TODO: How do i make this optional?
            //       Throws an error if case_sensitive is not listed in config file
            case_sensitive: false,
        }
    }
}

impl Vault {
    pub fn to_glob(&self) -> String {
        let _path = String::from(self.path.display().to_string());
        let mut _glob = shellexpand::full(_path.as_str()).unwrap();
        _glob.to_mut().push_str(std::path::MAIN_SEPARATOR_STR);
        _glob.to_mut().push_str(self.pattern.as_str());

        return _glob.to_string();
    }

    pub fn get_files(&self) -> Result<Paths, PatternError> {
        let glob_pattern = self.to_glob();
        let options: MatchOptions = MatchOptions {
            case_sensitive: self.case_sensitive,
            require_literal_leading_dot: false,
            require_literal_separator: false,
        };
        glob_with(glob_pattern.as_str(), options)
    }
}

// endregion vault

// region: Task

#[derive(Debug, Serialize, Deserialize)]
struct TaskIdConfig {
    type_name: IdType,
    length: u8
}

impl Default for TaskIdConfig {
    fn default() -> Self {
        Self {
            type_name: IdType::Nanoid,
            length: 23
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskConfig {
    id: TaskIdConfig,
}

impl Default for TaskConfig {
    fn default() -> Self {
        Self {
            id: TaskIdConfig::default()
        }
    }
}

// endregion task

// region: Config
#[derive(Debug, Serialize, Deserialize)]
#[allow(unused)]
pub struct Config {
    pub debug: bool,
    config_dir: PathBuf,
    pub vaults: Vec<Vault>,
    pub task: TaskConfig
}

impl Default for Config {
    fn default() -> Self {
        Self {
            debug: false,
            config_dir: Config::get_config_path(),
            vaults: Vec::<Vault>::default(),
            task: TaskConfig::default()

        }
    }
}

impl Config {
    pub fn new() -> Self {
        let config_file = Config::get_config_path();
        Config::load(APP_NAME, config_file.to_str()).unwrap()
    }

    pub fn load(&self, app: &str, conf: Option<&str>) -> Result<Self, ConfyError> {
        // TODO: Need to handle errors with the format, etc
        match confy::load(app, conf) {
            Ok(()) => (),
            Err(error) => match error {
                ConfyError::BadYamlData(_) => {
                    println!("The error was {:?}", error);
                    ()
                }
                default_handler(_) => {
                    error
                }
            }
        };
    }

    pub fn get_config_path() -> PathBuf {
        let strategy = choose_base_strategy().unwrap();
        let _config_dir = strategy.config_dir().join(APP_NAME);
        let _config_file_name = APP_NAME.to_string() + CONFIG_EXT;
        _config_dir.join(_config_file_name)
    }
}

// endregion Config

// region: Tests
#[cfg(test)]
mod tests {

    #[test]
    fn hello() {
        assert_eq!("hello", "hello");
    }
}
// endregion Tests
