
use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct VaultPath {
    name: String,
    path: std::path::PathBuf,
    pattern: String
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    debug: bool,
    vaults: Vec<VaultPath>
}

impl Settings {
    // TRA: Result<T, E> where T == operation succeeded type, E == operation failed type
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("settings/default.yml"))
            .add_source(Environment::with_prefix("VTASK"))
            .build()?;

            println!("debug: {:?}", s.get_bool("debug"));
            s.try_deserialize()
    }


}
