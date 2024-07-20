// region: Imports
//- std lib
use std::{default::Default, path::PathBuf, vec::Vec};

//- crates
use confy;
use etcetera::{choose_base_strategy, BaseStrategy};
use glob::{glob_with, MatchOptions, Paths, PatternError};
use serde_derive::{Deserialize, Serialize};
use shellexpand;

//- local
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
    class: String,
    length: u8,
}

impl Default for TaskIdConfig {
    fn default() -> Self {
        Self {
            class: String::from("nanoid"),
            length: 23,
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
            id: TaskIdConfig::default(),
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
    pub task: TaskConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            debug: false,
            config_dir: Config::get_config_path(),
            vaults: Vec::<Vault>::default(),
            task: TaskConfig::default(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let config_file = Config::get_config_path();
        let conf = match confy::load(APP_NAME, config_file.to_str()) {
            Ok(cfg) => { cfg },
            Err(err) => {
                println!("Error loading config: {:?}", err);
                Config::default()
            }
        };
        println!("id is {:?}", conf.task.id.class);
        conf
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
// endregion Tests
