use std::fs;
use std::path::Path;

use crate::build_scripts::config::types::{Function, Type};

pub(crate) fn generate_haskell_dispatch(functions: &[Function], out_dir: &Path) {
    let dest = out_dir.join("Dispatch.hs");

    let mut code = String::new();

    code.push_str("module Dispatch where\n\n");
    code.push_str("import qualified Data.ByteString as BS\n");
    code.push_str("import qualified Data.ByteString.Char8 as C8\n");
    code.push_str("import Codec\n");
    code.push_str("import Script\n\n");
    code.push_str("dispatchUserFunction :: BS.ByteString -> BS.ByteString -> IO BS.ByteString\n");
    code.push_str("dispatchUserFunction functionName input\n");

    for function in functions {
        code.push_str(&generate_dispatch_branch(function));
        code.push('\n');
    }

    code.push_str(
        "    | otherwise = error (\"Unknown Haskell function: \" ++ C8.unpack functionName)\n",
    );

    fs::write(&dest, code).expect("Failed to write Haskell dispatch module");
}

fn generate_dispatch_branch(function: &Function) -> String {
    let args_pattern = function
        .args
        .iter()
        .map(|arg| arg.r#type.haskell_value_pattern(&arg.name))
        .collect::<Vec<_>>()
        .join(", ");

    let call_args = function
        .args
        .iter()
        .map(|arg| arg.r#type.haskell_arg_expr(&arg.name))
        .collect::<Vec<_>>()
        .join(" ");

    let function_call = format!("{} {}", function.name, call_args);
    let converted_result = function.r#return.haskell_return_expr(&function_call);
    let encoder = function.r#return.haskell_encoder();

    format!(
        concat!(
            "    | functionName == C8.pack \"{name}\" =\n",
            "        case decodeValues input of\n",
            "            [{args_pattern}] -> pure ({encoder} ({converted_result}))\n",
            "            _ -> error \"Invalid arguments for function '{name}'\""
        ),
        name = function.name,
        args_pattern = args_pattern,
        encoder = encoder,
        converted_result = converted_result,
    )
}

impl Type {
    fn haskell_value_pattern(&self, name: &str) -> String {
        match self {
            Type::Int => format!("VInt {}", name),
            Type::Float => format!("VFloat {}", name),
            Type::Bool => format!("VBool {}", name),
            Type::String => format!("VString {}", name),
            Type::Bytes => format!("VBytes {}", name),
        }
    }

    fn haskell_encoder(&self) -> &'static str {
        match self {
            Type::Int => "encodeInt",
            Type::Float => "encodeFloat",
            Type::Bool => "encodeBool",
            Type::String => "encodeString",
            Type::Bytes => "encodeBytes",
        }
    }
}
