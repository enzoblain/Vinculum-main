use std::fs;
use std::path::Path;

use crate::build::parser::{Function, extract_functions};

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

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();

    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub(crate) fn to_snake_case(s: &str) -> String {
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

pub(crate) fn collect_haskell_modules_from_exports(
    exports_dir: &Path,
) -> Vec<(String, Vec<Function>)> {
    let mut modules = Vec::new();

    if let Ok(entries) = fs::read_dir(exports_dir) {
        collect_modules_recursive(
            &entries.flatten().map(|e| e.path()).collect::<Vec<_>>(),
            &mut modules,
        );
    }

    modules
}

fn collect_modules_recursive(
    paths: &[std::path::PathBuf],
    modules: &mut Vec<(String, Vec<Function>)>,
) {
    for path in paths {
        if path.is_file()
            && let Some("hs") = path.extension().and_then(|s| s.to_str())
            && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
            && let Ok(functions) = extract_functions(path)
            && !functions.is_empty()
        {
            modules.push((capitalize_first(stem), functions));
        } else if path.is_dir()
            && let Ok(entries) = fs::read_dir(path)
        {
            let sub_paths: Vec<_> = entries.flatten().map(|e| e.path()).collect();
            collect_modules_recursive(&sub_paths, modules);
        }
    }
}
