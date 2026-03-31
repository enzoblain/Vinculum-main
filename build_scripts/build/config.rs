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
    BuildConfig {
        lib_dir: env::var("HASKELL_LIB_DIR").unwrap_or_else(|_| "target/haskell".to_string()),
        lib_file: env::var("HASKELL_LIB_FILE").unwrap_or_else(|_| "HSmylib".to_string()),
        rts_lib: env::var("HASKELL_RTS_LIB").unwrap_or_else(|_| "HSrts-ghc9.6.7".to_string()),
        rts_dir: env::var("HASKELL_RTS_DIR").unwrap_or_else(|_| {
            "/Users/enzoblain/.ghcup/ghc/9.6.7/lib/ghc-9.6.7/lib/aarch64-osx-ghc-9.6.7".to_string()
        }),
        functions_file: env::var("HASKELL_FUNCTIONS_FILE")
            .unwrap_or_else(|_| "haskell_exports.toml".to_string()),
        haskell_dir: "build_scripts/haskell".to_string(),
        c_dir: "build_scripts/c".to_string(),
        user_functions_file: env::var("HASKELL_USER_FUNCTIONS_FILE")
            .unwrap_or_else(|_| "haskell/Script.hs".to_string()),
    }
}
