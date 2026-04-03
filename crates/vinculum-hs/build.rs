use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io;
use std::path::Path;

mod build_scripts;

use build_scripts::build::{compiler, linker, validator};
use build_scripts::codegen::{generate_functions_with_modules, generate_haskell_dispatch};
use build_scripts::utils::config::load_config;
use build_scripts::utils::helpers::{
    collect_file_modules, emit_rerun_if_changed, generate_user_functions_module,
    log_registered_functions,
};

fn main() {
    let config =
        load_config().unwrap_or_else(|e| panic!("Failed to load build configuration: {e}"));

    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let ffi_dir = Path::new("build_scripts/ffi");
    let fingerprint_path = Path::new(&config.lib_dir).join(".vinculum-input-fingerprint");

    let haskell_dir = Path::new(&config.functions_dir);

    assert!(
        haskell_dir.exists(),
        "Invalid configuration: HASKELL_DIR does not exist: {}",
        haskell_dir.display()
    );
    assert!(
        haskell_dir.is_dir(),
        "Invalid configuration: HASKELL_DIR is not a directory: {}",
        haskell_dir.display()
    );

    let target_haskell_dir = Path::new(&config.lib_dir);
    fs::create_dir_all(target_haskell_dir)
        .unwrap_or_else(|e| panic!("Failed to create Haskell build dir: {e}"));

    println!("cargo:rerun-if-changed=Cargo.toml");
    emit_rerun_if_changed(ffi_dir, &config.functions_dir);

    let current_fingerprint = fingerprint_inputs(&manifest_path, haskell_dir, ffi_dir)
        .unwrap_or_else(|e| panic!("Failed to fingerprint build inputs: {e}"));
    let existing_fingerprint = fs::read_to_string(&fingerprint_path).ok();

    let output_ext = validator::shared_library_extension();
    let output_file =
        target_haskell_dir.join(validator::library_filename(&config.lib_file, output_ext));
    let outputs_are_present = output_file.exists();

    if existing_fingerprint.as_deref() == Some(current_fingerprint.as_str()) && outputs_are_present
    {
        linker::emit_link_instructions(&config)
            .unwrap_or_else(|e| panic!("Failed to emit link instructions: {e}"));
        return;
    }

    let file_modules = collect_file_modules(haskell_dir, target_haskell_dir);

    let user_functions_content = generate_user_functions_module(&file_modules);
    let user_functions_path = target_haskell_dir.join("UserFunctions.hs");
    fs::write(&user_functions_path, user_functions_content)
        .unwrap_or_else(|e| panic!("Failed to write '{}': {e}", user_functions_path.display()));

    generate_functions_with_modules(&file_modules);

    let ffi_dir = Path::new("build_scripts/ffi");
    generate_haskell_dispatch(&file_modules, ffi_dir)
        .unwrap_or_else(|e| panic!("Failed to generate Haskell dispatch: {e}"));

    let lib_path = Path::new(&config.lib_dir);
    fs::create_dir_all(lib_path)
        .unwrap_or_else(|e| panic!("Failed to create Haskell library output directory: {e}"));

    validator::validate_library_dir(lib_path).unwrap_or_else(|e| panic!("{e}"));
    compiler::compile_haskell_library(ffi_dir, &user_functions_path, lib_path, &config.lib_file);
    validator::validate_main_library(&config).unwrap_or_else(|e| panic!("{e}"));
    validator::warn_if_rts_missing(&config);
    compiler::copy_rts_library(&config.rts_dir, &config.rts_lib, lib_path);
    linker::emit_link_instructions(&config)
        .unwrap_or_else(|e| panic!("Failed to emit link instructions: {e}"));

    fs::write(&fingerprint_path, current_fingerprint).unwrap_or_else(|e| {
        panic!(
            "Failed to write build fingerprint '{}': {e}",
            fingerprint_path.display()
        )
    });
    log_registered_functions(&file_modules);
}

fn fingerprint_inputs(
    manifest_path: &Path,
    haskell_dir: &Path,
    ffi_dir: &Path,
) -> io::Result<String> {
    let mut inputs = Vec::new();
    collect_fingerprint_inputs(manifest_path, &mut inputs)?;
    collect_fingerprint_inputs(haskell_dir, &mut inputs)?;
    collect_fingerprint_inputs(ffi_dir, &mut inputs)?;

    inputs.sort_by(|left, right| left.0.cmp(&right.0));

    let mut hasher = DefaultHasher::new();
    for (path, content) in inputs {
        path.hash(&mut hasher);
        content.hash(&mut hasher);
    }

    Ok(format!("{:016x}", hasher.finish()))
}

fn collect_fingerprint_inputs(path: &Path, inputs: &mut Vec<(String, Vec<u8>)>) -> io::Result<()> {
    if path.is_file() {
        if path.extension().and_then(|s| s.to_str()) == Some("hs")
            || path.file_name().and_then(|s| s.to_str()) == Some("Cargo.toml")
        {
            inputs.push((path.to_string_lossy().to_string(), fs::read(path)?));
        }

        return Ok(());
    }

    if !path.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        collect_fingerprint_inputs(&entry.path(), inputs)?;
    }

    Ok(())
}
