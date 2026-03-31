use std::path::PathBuf;
use std::{env, fs};

use crate::build_scripts::config::types::Function;

pub(crate) fn generate_functions(functions: &[Function]) {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set"));
    let dest = out_dir.join("generated_functions.rs");

    let mut code = String::new();

    for function in functions {
        code.push_str(&generate_function(function));
        code.push('\n');
    }

    fs::write(&dest, code).expect("Failed to write generated Rust bindings");
}

fn generate_function(function: &Function) -> String {
    let args_sig = function
        .args
        .iter()
        .map(|arg| format!("{}: {}", arg.name, arg.r#type.rust_type()))
        .collect::<Vec<_>>()
        .join(", ");

    let args_values = function
        .args
        .iter()
        .map(|arg| arg.r#type.rust_value_ctor(&arg.name))
        .collect::<Vec<_>>()
        .join(", ");

    let return_type = function.r#return.rust_type();
    let converter = function.r#return.return_converter();

    format!(
        "pub fn {name}({args_sig}) -> {return_type} {{
            let result = call_haskell_typed(\"{name}\", &[{args_values}]);
            {converter}(result)
        }}",
        name = function.name,
        args_sig = args_sig,
        return_type = return_type,
        args_values = args_values,
        converter = converter,
    )
}
