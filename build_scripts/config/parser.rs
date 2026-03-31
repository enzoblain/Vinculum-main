use std::fs;
use std::path::Path;

use super::types::{Config, Function};
use super::validator::validate_functions;

pub(crate) fn parse_haskell_functions(path: impl AsRef<Path>) -> Vec<Function> {
    let content = fs::read_to_string(path.as_ref()).expect("Failed to read configuration file");

    let config: Config = toml::from_str(&content).expect("Invalid TOML configuration");

    validate_functions(&config.functions);

    config.functions
}
