use std::path::PathBuf;

use glob::{glob_with, MatchOptions, Paths, PatternError};
use serde_derive::{Deserialize, Serialize};
use shellexpand;

#[derive(Debug, Serialize, Deserialize)]
#[allow(unused)]
pub struct Vault {
    name: String,
    path: std::path::PathBuf,
    pattern: String,
    case_sensitive: bool,
}

impl std::default::Default for Vault {
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
