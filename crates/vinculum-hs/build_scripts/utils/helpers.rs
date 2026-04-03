use std::fs;
use std::path::Path;

use crate::build_scripts::parser::{extract_functions, Function};

pub(crate) fn collect_file_modules(
    src_dir: &Path,
    dest_dir: &Path,
) -> Vec<(String, Vec<Function>)> {
    let entries = fs::read_dir(src_dir)
        .unwrap_or_else(|e| panic!("Failed to read directory '{}': {e}", src_dir.display()));

    let mut modules = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();

        let is_haskell_file =
            path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("hs");

        if !is_haskell_file {
            continue;
        }

        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };

        let dest = dest_dir.join(format!("{stem}.hs"));
        fs::copy(&path, &dest)
            .unwrap_or_else(|e| panic!("Failed to copy '{}': {e}", path.display()));

        let functions = extract_functions(&path)
            .unwrap_or_else(|e| panic!("Failed to parse '{}': {e}", path.display()));

        if !functions.is_empty() {
            modules.push((capitalize_first(stem), functions));
        }
    }

    modules
}

pub(crate) fn generate_user_functions_module(file_modules: &[(String, Vec<Function>)]) -> String {
    if file_modules.is_empty() {
        return "module UserFunctions where\n".to_string();
    }

    let mut exports = Vec::new();
    let mut imports = Vec::new();
    let mut wrappers = Vec::new();

    for (module_name, functions) in file_modules {
        imports.push(format!("import qualified {module_name} as {module_name}"));

        for function in functions {
            let alias = format!("{}_{}", to_snake_case(module_name), function.name);

            exports.push(alias.clone());
            wrappers.push(format!("{alias} = {module_name}.{}", function.name));
        }
    }

    format!(
        "module UserFunctions (\n    {}\n) where\n\n{}\n\n{}\n",
        exports.join(", "),
        imports.join("\n"),
        wrappers.join("\n"),
    )
}

pub(crate) fn log_registered_functions(file_modules: &[(String, Vec<Function>)]) {
    for (module_name, functions) in file_modules {
        for function in functions {
            println!(
                "cargo:warning=Registered Haskell function: {} from module {module_name}",
                function.name
            );
        }
    }
}

pub fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();

    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();

    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            result.push('_');
            result.push_str(&ch.to_lowercase().to_string());
        } else {
            result.push(ch.to_lowercase().next().unwrap_or(ch));
        }
    }

    result
}
