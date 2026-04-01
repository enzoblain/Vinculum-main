use std::fs;
use std::path::Path;

use crate::build_scripts::parser::types::{Function, Type};
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
        .map(|arg| arg.r#type.haskell_value_pattern(&arg.name))
        .collect::<Vec<_>>()
        .join(", ");

    let call_args = function
        .args
        .iter()
        .map(|arg| arg.r#type.haskell_arg_expr(&arg.name))
        .collect::<Vec<_>>()
        .join(" ");

    let qualified_name = format!("{}_{}", to_snake_case(module_name), function.name);

    let function_call = if call_args.is_empty() {
        qualified_name.clone()
    } else {
        format!("{} {}", qualified_name, call_args)
    };

    let converted_result = function.r#return.haskell_return_expr(&function_call);
    let encoder = function.r#return.haskell_encoder();

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

impl Type {
    pub(crate) fn haskell_value_pattern(&self, name: &str) -> String {
        match self {
            Type::Int8 => format!("VInt8 {}", name),
            Type::Int16 => format!("VInt16 {}", name),
            Type::Int32 => format!("VInt32 {}", name),
            Type::Int64 => format!("VInt64 {}", name),
            Type::Word8 => format!("VWord8 {}", name),
            Type::Word16 => format!("VWord16 {}", name),
            Type::Word32 => format!("VWord32 {}", name),
            Type::Word64 => format!("VWord64 {}", name),
            Type::Float32 => format!("VFloat32 {}", name),
            Type::Float64 => format!("VFloat64 {}", name),
            Type::Bool => format!("VBool {}", name),
            Type::Char => format!("VChar {}", name),
            Type::String => format!("VString {}", name),
            Type::Bytes => format!("VBytes {}", name),
            Type::Maybe(_) => format!("VOption {}", name),
            Type::Vec(_) => format!("VVec {}", name),
        }
    }

    pub(crate) fn haskell_value_constructor(&self) -> String {
        match self {
            Type::Int8 => "VInt8".to_string(),
            Type::Int16 => "VInt16".to_string(),
            Type::Int32 => "VInt32".to_string(),
            Type::Int64 => "VInt64".to_string(),
            Type::Word8 => "VWord8".to_string(),
            Type::Word16 => "VWord16".to_string(),
            Type::Word32 => "VWord32".to_string(),
            Type::Word64 => "VWord64".to_string(),
            Type::Float32 => "VFloat32".to_string(),
            Type::Float64 => "VFloat64".to_string(),
            Type::Bool => "VBool".to_string(),
            Type::Char => "VChar".to_string(),
            Type::String => "VString".to_string(),
            Type::Bytes => "VBytes".to_string(),
            Type::Maybe(_) => unreachable!("Maybe should not be used as a value constructor"),
            Type::Vec(_) => unreachable!("Vec should not be used as a value constructor"),
        }
    }

    pub(crate) fn haskell_encoder(&self) -> String {
        match self {
            Type::Int8 => "encodeInt8".to_string(),
            Type::Int16 => "encodeInt16".to_string(),
            Type::Int32 => "encodeInt32".to_string(),
            Type::Int64 => "encodeInt64".to_string(),
            Type::Word8 => "encodeWord8".to_string(),
            Type::Word16 => "encodeWord16".to_string(),
            Type::Word32 => "encodeWord32".to_string(),
            Type::Word64 => "encodeWord64".to_string(),
            Type::Float32 => "encodeFloat32".to_string(),
            Type::Float64 => "encodeFloat64".to_string(),
            Type::Bool => "encodeBool".to_string(),
            Type::Char => "encodeChar".to_string(),
            Type::String => "encodeString".to_string(),
            Type::Bytes => "encodeBytes".to_string(),
            Type::Maybe(inner) => format!("encodeOptionWith {}", inner.haskell_value_constructor()),
            Type::Vec(_) => "encodeVec".to_string(),
        }
    }
}
