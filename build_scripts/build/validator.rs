use std::path::Path;

use super::config::BuildConfig;

pub(crate) fn validate_functions_file(path: &Path) {
    if !path.exists() {
        panic!(
            "Invalid configuration: HASKELL_FUNCTIONS_FILE does not exist: {}",
            path.display()
        );
    }

    if !path.is_file() {
        panic!(
            "Invalid configuration: HASKELL_FUNCTIONS_FILE is not a file: {}",
            path.display()
        );
    }
}

pub(crate) fn validate_library_dir(path: &Path) {
    if !path.exists() {
        panic!(
            "Invalid configuration: HASKELL_LIB_DIR does not exist: {}",
            path.display()
        );
    }

    if !path.is_dir() {
        panic!(
            "Invalid configuration: HASKELL_LIB_DIR is not a directory: {}",
            path.display()
        );
    }
}

pub(crate) fn shared_library_extension() -> &'static str {
    if cfg!(target_os = "windows") {
        "dll"
    } else if cfg!(target_os = "macos") {
        "dylib"
    } else {
        "so"
    }
}

pub(crate) fn library_filename(name: &str, ext: &str) -> String {
    if cfg!(target_os = "windows") {
        format!("{}.{}", name, ext)
    } else {
        format!("lib{}.{}", name, ext)
    }
}

pub(crate) fn validate_main_library(config: &BuildConfig) {
    let lib_path = Path::new(&config.lib_dir);
    let ext = shared_library_extension();
    let lib_filename = library_filename(&config.lib_file, ext);
    let full_lib_path = lib_path.join(&lib_filename);

    if !full_lib_path.exists() {
        panic!(
            "Linking error: Haskell library not found at expected path: {}",
            full_lib_path.display()
        );
    }

    if !full_lib_path.is_file() {
        panic!(
            "Linking error: Expected Haskell library is not a file: {}",
            full_lib_path.display()
        );
    }
}

pub(crate) fn warn_if_rts_missing(config: &BuildConfig) {
    let lib_path = Path::new(&config.lib_dir);
    let ext = shared_library_extension();
    let rts_filename = library_filename(&config.rts_lib, ext);
    let rts_path = lib_path.join(&rts_filename);

    if !rts_path.exists() {
        println!(
            "cargo:warning=Haskell RTS library '{}' not found in '{}'. Ensure the GHC runtime library path is correctly configured.",
            rts_filename, config.lib_dir
        );
    }
}
