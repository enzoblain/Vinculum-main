use std::fs;
use std::path::Path;

use super::errors::ParseError;
use super::types::{Arg, Function, Type};
use super::validator::validate_functions;

#[derive(Debug)]
struct HsArg {
    name: String,
    r#type: String,
}

#[derive(Debug)]
struct HsFunction {
    name: String,
    args: Vec<HsArg>,
    return_type: String,
}

pub(crate) fn parse_haskell_functions(path: impl AsRef<Path>) -> Result<Vec<Function>, ParseError> {
    let path = path.as_ref();

    let content = fs::read_to_string(path).map_err(|source| ParseError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;

    let mut functions = Vec::new();
    let mut current: Option<String> = None;

    for raw_line in content.lines() {
        let line = strip_comment(raw_line).trim_end();

        if line.trim().is_empty() {
            continue;
        }

        if is_signature(line) {
            flush(&mut current, &mut functions)?;
            current = Some(line.trim().to_string());
            continue;
        }

        if let Some(buf) = &mut current {
            if raw_line.starts_with(char::is_whitespace) {
                buf.push(' ');
                buf.push_str(line.trim());
                continue;
            }

            flush(&mut current, &mut functions)?;
        }
    }

    flush(&mut current, &mut functions)?;

    let typed_functions = functions
        .into_iter()
        .map(convert_to_typed_function)
        .collect::<Result<Vec<_>, ParseError>>()?;

    validate_functions(&typed_functions).map_err(ParseError::Validation)?;

    Ok(typed_functions)
}

fn strip_comment(line: &str) -> &str {
    line.split_once("--").map(|(code, _)| code).unwrap_or(line)
}

fn is_signature(line: &str) -> bool {
    line.contains("::")
}

fn flush(current: &mut Option<String>, functions: &mut Vec<HsFunction>) -> Result<(), ParseError> {
    if let Some(buf) = current.take() {
        let function = parse_function_signature(&buf)?;
        functions.push(function);
    }

    Ok(())
}

fn parse_function_signature(signature: &str) -> Result<HsFunction, ParseError> {
    let (name, type_expr) = signature
        .split_once("::")
        .ok_or_else(|| ParseError::InvalidSignature(signature.to_string()))?;

    let name = name.trim().to_string();

    let parts: Vec<&str> = type_expr
        .split("->")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if parts.is_empty() {
        return Err(ParseError::InvalidSignature(signature.to_string()));
    }

    if parts.len() == 1 {
        return Ok(HsFunction {
            name,
            args: Vec::new(),
            return_type: parts[0].to_string(),
        });
    }

    let return_type = parts
        .last()
        .ok_or_else(|| ParseError::InvalidSignature(signature.to_string()))?
        .to_string();

    let args = parts[..parts.len() - 1]
        .iter()
        .enumerate()
        .map(|(i, ty)| HsArg {
            name: format!("arg{}", i + 1),
            r#type: ty.to_string(),
        })
        .collect();

    Ok(HsFunction {
        name,
        args,
        return_type,
    })
}

fn convert_to_typed_function(hs_func: HsFunction) -> Result<Function, ParseError> {
    let return_type = parse_haskell_type(&hs_func.return_type)?;

    let args = hs_func
        .args
        .into_iter()
        .map(|arg| {
            Ok(Arg {
                name: arg.name,
                r#type: parse_haskell_type(&arg.r#type)?,
            })
        })
        .collect::<Result<Vec<_>, ParseError>>()?;

    Ok(Function {
        name: hs_func.name,
        args,
        r#return: return_type,
    })
}

fn parse_haskell_type(type_str: &str) -> Result<Type, ParseError> {
    let type_str = type_str.trim();

    if let Some(inner) = type_str.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
        let inner_type = parse_haskell_type(inner)?;
        return Ok(Type::Vec(Box::new(inner_type)));
    }

    if let Some(inner) = type_str.strip_prefix("Maybe") {
        let inner = inner.trim();
        let inner_type = parse_haskell_type(inner)?;
        return Ok(Type::Maybe(Box::new(inner_type)));
    }

    match type_str {
        "Int8" => Ok(Type::Int8),
        "Int16" => Ok(Type::Int16),
        "Int32" => Ok(Type::Int32),
        "Int64" => Ok(Type::Int64),
        "Word8" => Ok(Type::Word8),
        "Word16" => Ok(Type::Word16),
        "Word32" => Ok(Type::Word32),
        "Word64" => Ok(Type::Word64),
        "Float32" | "Float" => Ok(Type::Float32),
        "Float64" | "Double" => Ok(Type::Float64),
        "Bool" => Ok(Type::Bool),
        "Char" => Ok(Type::Char),
        "String" => Ok(Type::String),
        "ByteString" | "Bytes" => Ok(Type::Bytes),
        _ => Err(ParseError::UnknownType(type_str.to_string())),
    }
}
