use std::path::Path;

use super::config::BuildConfig;

pub(crate) fn emit_link_instructions(config: &BuildConfig) {
    println!("cargo:rustc-link-search=native={}", config.lib_dir);
    println!("cargo:rustc-link-search=native={}", config.rts_dir);
    println!("cargo:rustc-link-lib=dylib={}", config.lib_file);
    println!("cargo:rustc-link-lib=dylib={}", config.rts_lib);

    if cfg!(target_os = "macos") {
        let lib_dir = Path::new(&config.lib_dir)
            .canonicalize()
            .expect("Failed to canonicalize HASKELL_LIB_DIR");

        let rts_dir = Path::new(&config.rts_dir)
            .canonicalize()
            .expect("Failed to canonicalize HASKELL_RTS_DIR");

        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir.display());
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", rts_dir.display());
    }
}

pub(crate) fn emit_rerun_instructions(functions_file: &str) {
    println!("cargo:rerun-if-env-changed=HASKELL_LIB_DIR");
    println!("cargo:rerun-if-env-changed=HASKELL_LIB_FILE");
    println!("cargo:rerun-if-env-changed=HASKELL_RTS_LIB");
    println!("cargo:rerun-if-env-changed=HASKELL_FUNCTIONS_FILE");
    println!("cargo:rerun-if-changed={}", functions_file);
}
