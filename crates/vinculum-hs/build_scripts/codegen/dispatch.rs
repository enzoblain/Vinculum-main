use std::fs;
use std::path::Path;

use crate::build_scripts::parser::Function;
use crate::build_scripts::utils::to_snake_case;

pub(crate) fn generate_haskell_dispatch(
    file_modules: &[(String, Vec<Function>)],
    out_dir: &Path,
) -> Result<(), std::io::Error> {
    let dest = out_dir.join("Dispatch.hs");

    let mut code = String::new();

    code.push_str("module Dispatch where\n\n");
    code.push_str("import qualified Data.ByteString as BS\n");
    code.push_str("import qualified Data.ByteString.Char8 as C8\n");
    code.push_str("import Codec\n");
    code.push_str("import UserFunctions\n\n");
    code.push_str("dispatchUserFunction :: BS.ByteString -> BS.ByteString -> IO BS.ByteString\n");
    code.push_str("dispatchUserFunction functionName input\n");

    for (module_name, functions) in file_modules {
        for function in functions {
            code.push_str(&generate_dispatch_branch(function, module_name));
            code.push('\n');
        }
    }

    code.push_str(
        "    | otherwise = error (\"Unknown Haskell function: \" ++ C8.unpack functionName)\n",
    );

    fs::write(&dest, code)
}

fn generate_dispatch_branch(function: &Function, module_name: &str) -> String {
    let args_pattern = function
        .args
        .iter()
        .map(|arg| arg.r#type.haskell_pattern(&arg.name))
        .collect::<Vec<_>>()
        .join(", ");

    let call_args = function
        .args
        .iter()
        .map(|arg| arg.r#type.to_haskell_value(&arg.name))
        .collect::<Vec<_>>()
        .join(" ");

    let qualified_name = format!("{}_{}", to_snake_case(module_name), function.name);

    let function_call = if call_args.is_empty() {
        qualified_name.clone()
    } else {
        format!("{} {}", qualified_name, call_args)
    };

    let converted_result = function.r#return.from_haskell_value(&function_call);
    let encoder = function.r#return.haskell_encode_fn();

    format!(
        concat!(
            "    | functionName == C8.pack \"{qualified_name}\" =\n",
            "        case decodeValues input of\n",
            "            [{args_pattern}] -> pure ({encoder} ({converted_result}))\n",
            "            _ -> error \"Invalid arguments for function '{qualified_name}'\""
        ),
        qualified_name = qualified_name,
        args_pattern = args_pattern,
        encoder = encoder,
        converted_result = converted_result,
    )
}
