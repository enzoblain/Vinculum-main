use std::fs;
use std::path::Path;

mod build_scripts;

use build_scripts::build::{compiler, config, linker, validator};
use build_scripts::codegen::{dispatch, functions as codegen_functions};
use build_scripts::parser::extract_functions;
use build_scripts::utils::{capitalize_first, to_snake_case};

fn main() {
    let config = config::load_config()
        .unwrap_or_else(|e| panic!("Failed to load build configuration: {}", e));

    let haskell_dir = Path::new(&config.functions_dir);

    if !haskell_dir.exists() {
        panic!(
            "Invalid configuration: HASKELL_DIR does not exist: {}",
            haskell_dir.display()
        );
    }

    if !haskell_dir.is_dir() {
        panic!(
            "Invalid configuration: HASKELL_DIR is not a directory: {}",
            haskell_dir.display()
        );
    }

    let target_haskell_dir = Path::new(&config.lib_dir);
    fs::create_dir_all(target_haskell_dir)
        .unwrap_or_else(|e| panic!("Failed to create Haskell build dir: {}", e));

    let mut file_modules = Vec::new();

    let entries = fs::read_dir(haskell_dir).unwrap_or_else(|e| {
        panic!(
            "Failed to read directory '{}': {}",
            haskell_dir.display(),
            e
        )
    });

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_file()
            && path.extension().and_then(|s| s.to_str()) == Some("hs")
            && let Some(file_name) = path.file_stem().and_then(|s| s.to_str())
        {
            let target_file = target_haskell_dir.join(format!("{}.hs", file_name));

            fs::copy(&path, &target_file)
                .unwrap_or_else(|e| panic!("Failed to copy '{}': {}", path.display(), e));

            let file_functions = extract_functions(&path)
                .unwrap_or_else(|e| panic!("Failed to parse '{}': {}", path.display(), e));

            if !file_functions.is_empty() {
                let module_name = capitalize_first(file_name);
                file_modules.push((module_name.clone(), file_functions));
            }
        }
    }

    let user_functions_content = if !file_modules.is_empty() {
        let mut exports = Vec::new();
        let mut imports = Vec::new();
        let mut wrappers = Vec::new();

        for (module_name, functions) in &file_modules {
            imports.push(format!(
                "import qualified {} as {}",
                module_name, module_name
            ));

            for function in functions {
                let qualified_name = format!("{}_{}", to_snake_case(module_name), function.name);
                exports.push(qualified_name.clone());
                wrappers.push(format!(
                    "{} = {}.{}",
                    qualified_name, module_name, function.name
                ));
            }
        }

        let exports_str = exports.join(", ");
        let imports_str = imports.join("\n");
        let wrappers_str = wrappers.join("\n");

        format!(
            "module UserFunctions (\n    {}\n) where\n\n{}\n\n{}\n",
            exports_str, imports_str, wrappers_str
        )
    } else {
        "module UserFunctions where\n".to_string()
    };

    let user_functions_path = target_haskell_dir.join("UserFunctions.hs");
    fs::write(&user_functions_path, user_functions_content)
        .unwrap_or_else(|e| panic!("Failed to write '{}': {}", user_functions_path.display(), e));

    codegen_functions::generate_functions_with_modules(&file_modules);

    let haskell_build_dir = Path::new("build_scripts/haskell");
    let c_dir = Path::new("build_scripts/c");

    dispatch::generate_haskell_dispatch(&file_modules, haskell_build_dir)
        .unwrap_or_else(|e| panic!("Failed to generate Haskell dispatch: {}", e));

    let lib_path = Path::new(&config.lib_dir);
    fs::create_dir_all(lib_path)
        .unwrap_or_else(|e| panic!("Failed to create Haskell library output directory: {}", e));

    validator::validate_library_dir(lib_path).unwrap_or_else(|e| panic!("{e}"));

    compiler::compile_haskell_library(
        haskell_build_dir,
        c_dir,
        &user_functions_path,
        lib_path,
        &config.lib_file,
    );

    validator::validate_main_library(&config).unwrap_or_else(|e| panic!("{e}"));

    validator::warn_if_rts_missing(&config);

    compiler::copy_rts_library(&config.rts_dir, &config.rts_lib, lib_path);

    linker::emit_link_instructions(&config)
        .unwrap_or_else(|e| panic!("Failed to emit link instructions: {}", e));

    linker::emit_rerun_instructions(&config.functions_dir);

    println!(
        "cargo:rerun-if-changed={}",
        Path::new(haskell_build_dir).join("Runtime.hs").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        Path::new(haskell_build_dir).join("Codec.hs").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        Path::new(haskell_build_dir).join("Dispatch.hs").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        Path::new(&config.functions_dir).display()
    );

    for (module_name, functions) in &file_modules {
        for function in functions {
            println!(
                "cargo:warning=Registered Haskell function: {} from module {}",
                function.name, module_name
            );
        }
    }
}
