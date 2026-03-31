use std::fs;
use std::path::Path;

use super::build::compiler::{compile_haskell_library, copy_rts_library};
use super::build::config::load_config;
use super::build::linker::{emit_link_instructions, emit_rerun_instructions};
use super::build::validator::{
    validate_functions_file, validate_library_dir, validate_main_library, warn_if_rts_missing,
};
use super::codegen::dispatch::generate_haskell_dispatch;
use super::codegen::functions::generate_functions;
use super::config::parser::parse_haskell_functions;

pub(crate) fn run() {
    let config = load_config();

    let functions_path = Path::new(&config.functions_file);
    validate_functions_file(functions_path);

    let functions = parse_haskell_functions(functions_path);

    generate_functions(&functions);

    let haskell_dir = Path::new(&config.haskell_dir);
    let c_dir = Path::new(&config.c_dir);
    let user_functions_path = Path::new(&config.user_functions_file);
    generate_haskell_dispatch(&functions, haskell_dir);

    let lib_path = Path::new(&config.lib_dir);
    fs::create_dir_all(lib_path).expect("Failed to create Haskell library output directory");

    validate_library_dir(lib_path);

    compile_haskell_library(
        haskell_dir,
        c_dir,
        user_functions_path,
        lib_path,
        &config.lib_file,
    );

    validate_main_library(&config);
    warn_if_rts_missing(&config);

    copy_rts_library(&config.rts_dir, &config.rts_lib, lib_path);

    emit_link_instructions(&config);
    emit_rerun_instructions(&config.functions_file);

    println!(
        "cargo:rerun-if-changed={}",
        haskell_dir.join("Runtime.hs").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        haskell_dir.join("Codec.hs").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        haskell_dir.join("Dispatch.hs").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        Path::new(&config.user_functions_file).display()
    );

    for function in &functions {
        println!(
            "cargo:warning=Registered Haskell function: {}",
            function.name
        );
    }
}
