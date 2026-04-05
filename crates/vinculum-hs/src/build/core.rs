use std::{env, fs};

use crate::build::compiler::{cabal, config, utils};
use crate::build::codegen::{generate_functions_with_modules, generate_haskell_dispatch};
use crate::build::utils::helpers::{
    collect_haskell_modules_from_exports, generate_user_functions_module,
};

pub fn build() -> Result<(), String> {
    let haskell_config = config::load_haskell_config()?;

    println!("cargo:rerun-if-changed=Cargo.toml");
    println!(
        "cargo:rerun-if-changed={}",
        haskell_config.exports_dir.display()
    );

    let file_modules = collect_haskell_modules_from_exports(&haskell_config.exports_dir);

    let vinculum_dir = haskell_config.exports_dir.join("vinculum");
    let generated_dir = vinculum_dir.join("generated");

    if !file_modules.is_empty() {
        let user_functions_content = generate_user_functions_module(&file_modules);
        let user_functions_path = generated_dir.join("UserFunctions.hs");

        let dispatch_content = generate_haskell_dispatch(&file_modules);
        let dispatch_path = generated_dir.join("dispatch.hs");

        fs::create_dir_all(&vinculum_dir).map_err(|e| format!("Failed to create vinculum: {e}"))?;
        fs::create_dir_all(&generated_dir)
            .map_err(|e| format!("Failed to create generated: {e}"))?;

        fs::write(&user_functions_path, user_functions_content)
            .map_err(|e| format!("Failed to write UserFunctions.hs: {e}"))?;
        fs::write(&dispatch_path, dispatch_content)
            .map_err(|e| format!("Failed to write Dispatch.hs: {e}"))?;

        generate_functions_with_modules(&file_modules);
    }

    utils::prepare_vinculum(&haskell_config.exports_dir)?;

    let cabal_path = cabal::find_cabal()?;

    cabal::build_haskell_dll(
        &cabal_path,
        &haskell_config.cabal_file,
        &haskell_config.foreign_library,
    )?;

    let haskell_dll_dir = env::var("CARGO_TARGET_DIR")
        .map(|dir| format!("{}/haskell", dir))
        .or_else(|_| {
            env::var("CARGO_MANIFEST_DIR").map(|dir| format!("{}/../../target/haskell", dir))
        })
        .unwrap_or_else(|_| "target/haskell".to_string());

    println!("cargo:rustc-link-search=native={}", haskell_dll_dir);
    println!(
        "cargo:rustc-link-lib=dylib={}",
        haskell_config.foreign_library
    );

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", haskell_dll_dir);

    Ok(())
}
