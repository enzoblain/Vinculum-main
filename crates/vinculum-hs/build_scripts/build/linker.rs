use std::io;
use std::path::Path;

use crate::build_scripts::utils::config::BuildConfig;

pub(crate) fn emit_link_instructions(config: &BuildConfig) -> Result<(), io::Error> {
    println!("cargo:rustc-link-search=native={}", config.lib_dir);
    println!("cargo:rustc-link-search=native={}", config.rts_dir);

    println!("cargo:rustc-link-lib=dylib={}", config.lib_file);
    println!("cargo:rustc-link-lib=dylib={}", config.rts_lib);

    let lib_dir = Path::new(&config.lib_dir).canonicalize()?;
    let rts_dir = Path::new(&config.rts_dir).canonicalize()?;

    if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir.display());
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", rts_dir.display());
    } else if cfg!(target_os = "windows") {
        println!("cargo:warning=On Windows, ensure DLLs are in PATH or next to the executable.");
    }

    Ok(())
}
