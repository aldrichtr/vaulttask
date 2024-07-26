
use std::{default::Default, path::PathBuf};

use glob::{glob_with, MatchOptions, Paths, PatternError};
use serde_derive::{Deserialize, Serialize};
use shellexpand;

/// # Vault configuration
///
/// `VaultConfig` is the `vaults:` section of the configuration
/// and identify where `vtask` should look for files.
#[derive(Debug, Serialize, Deserialize)]
#[allow(unused)]
pub struct VaultConfig {
    name: String,
    path: PathBuf,
    pattern: String,
    case_sensitive: bool,
}

impl Default for VaultConfig {
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

impl VaultConfig {
    /// Expand the path and then combine the `path` with the `pattern`
    fn to_glob(&self) -> String {
        // start with the `path` part of the vault config
        let path = self.path.to_str().unwrap();
        // do any tilde or env var expansion
        let path = shellexpand::full(path).unwrap();

        // add the pattern to the end
        let mut path = path.to_string();
        path.push_str(std::path::MAIN_SEPARATOR_STR);
        path.push_str(self.pattern.as_str());
        // return the path + pattern
        path
    }

    pub fn get_files(&self) -> Result<Paths, PatternError> {
        let glob = self.to_glob();
        let options: MatchOptions = MatchOptions {
            case_sensitive: self.case_sensitive,
            require_literal_leading_dot: false,
            require_literal_separator: false,
        };
        glob_with(glob.as_ref(), options)
    }
}
