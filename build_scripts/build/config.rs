use crate::build_scripts::build::compiler::{find_rts_dir, find_rts_lib};
use std::env;

pub struct BuildConfig {
    pub lib_dir: String,
    pub lib_file: String,
    pub rts_lib: String,
    pub rts_dir: String,
    pub functions_file: String,
    pub haskell_dir: String,
    pub c_dir: String,
    pub user_functions_file: String,
}

pub fn load_config() -> BuildConfig {
    let rts_dir = find_rts_dir();
    let rts_lib = find_rts_lib(&rts_dir);

    BuildConfig {
        lib_dir: env::var("HASKELL_LIB_DIR").unwrap_or_else(|_| "target/haskell".to_string()),
        lib_file: env::var("HASKELL_LIB_FILE").unwrap_or_else(|_| "HSmylib".to_string()),
        rts_lib,
        rts_dir,
        functions_file: env::var("HASKELL_FUNCTIONS_FILE")
            .unwrap_or_else(|_| "haskell_exports.toml".to_string()),
        haskell_dir: "build_scripts/haskell".to_string(),
        c_dir: "build_scripts/c".to_string(),
        user_functions_file: env::var("HASKELL_USER_FUNCTIONS_FILE")
            .unwrap_or_else(|_| "haskell/Script.hs".to_string()),
    }
}
