// region Imports

pub mod vault;

use crate::config::vault::Vault;

use std::path::PathBuf;
use std::vec::Vec;

use etcetera::{choose_base_strategy, BaseStrategy};
use serde_derive::{Deserialize, Serialize};
// endregion Imports

const APP_NAME: &str = "vtask";
const CONFIG_EXT: &str = ".config";

// region VaultPath
#[derive(Debug, Serialize, Deserialize)]
#[allow(unused)]
pub struct Config {
    pub debug: bool,
    pub vaults: Vec<Vault>,
    config_dir: PathBuf,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            debug: false,
            vaults: Vec::<Vault>::default(),
            config_dir: PathBuf::new(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let config_file = Config::get_config_path();
        Config::load(APP_NAME, config_file.to_str())
    }

    pub fn load(app: &str, conf: Option<&str>) -> Self {
        // TODO: Need to handle errors with the format, etc
        confy::load(app, conf).unwrap()
    }

    pub fn get_config_path() -> PathBuf {
        let strategy = choose_base_strategy().unwrap();
        let _config_dir = strategy.config_dir().join(APP_NAME);
        let _config_file_name = APP_NAME.to_string() + CONFIG_EXT;
        _config_dir.join(_config_file_name)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn hello() {
        assert_eq!("hello", "hello");
    }
}
