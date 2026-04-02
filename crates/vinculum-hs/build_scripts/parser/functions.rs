use std::mem;

use super::args::Arg;
use super::errors::ParseError;
use super::types::HaskellType;
use super::utils::is_rust_keyword;

pub(crate) struct Function {
    pub(crate) description: Vec<String>,
    pub(crate) name: String,
    pub(crate) args: Vec<Arg>,
    pub(crate) r#return: HaskellType,
}

#[derive(Default)]
pub(crate) struct FunctionBuffer {
    pub(crate) description: Vec<String>,
    pub(crate) signature: String,
    pub(crate) args: Vec<String>,
}

impl TryInto<Function> for FunctionBuffer {
    type Error = ParseError;

    fn try_into(self) -> Result<Function, Self::Error> {
        let signature = self.signature;
        let name_ref = signature
            .split_whitespace()
            .next()
            .ok_or(ParseError::EmptySignature)?;

        if !is_valid_rust_identifier(name_ref) {
            return Err(ParseError::InvalidFunctionName {
                name: name_ref.to_string(),
                signature,
            });
        }

        if is_rust_keyword(name_ref) {
            return Err(ParseError::ReservedRustKeyword {
                name: name_ref.to_string(),
            });
        }

        let (_, raw_types) =
            signature
                .split_once(":: ")
                .ok_or(ParseError::MissingHaskellTypeAnnotation {
                    signature: signature.clone(),
                })?;

        let parts: Vec<&str> = raw_types.split("->").map(str::trim).collect();

        let (return_type, arg_types) =
            parts
                .split_last()
                .ok_or(ParseError::MissingReturnHaskellType {
                    signature: signature.clone(),
                })?;

        let args = if self.args.is_empty() {
            arg_types
                .iter()
                .enumerate()
                .map(|(i, t)| Ok(Arg::new(format!("arg{}", i), HaskellType::try_from(*t)?)))
                .collect::<Result<Vec<_>, ParseError>>()?
        } else {
            if self.args.len() != arg_types.len() {
                return Err(ParseError::ArgumentCountMismatch {
                    expected: arg_types.len(),
                    found: self.args.len(),
                    signature: signature.clone(),
                });
            }

            self.args
                .into_iter()
                .zip(arg_types.iter())
                .map(|(name, t)| Ok(Arg::new(name, HaskellType::try_from(*t)?)))
                .collect::<Result<Vec<_>, ParseError>>()?
        };

        Ok(Function {
            description: self.description,
            name: name_ref.to_string(),
            args,
            r#return: HaskellType::try_from(*return_type)?,
        })
    }
}

#[inline]
fn is_valid_rust_identifier(name: &str) -> bool {
    let mut chars = name.bytes();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == b'_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == b'_')
}

pub(crate) fn push_function(
    functions: &mut Vec<Function>,
    buf: &mut FunctionBuffer,
) -> Result<(), ParseError> {
    let function = mem::take(buf).try_into()?;

    functions.push(function);

    Ok(())
}

pub(crate) fn is_signature(code_line: &str) -> Option<(&str, bool)> {
    let line = code_line.trim();

    if line.contains("::") {
        Some((line, true))
    } else if line.contains("->") {
        Some((line, false))
    } else {
        None
    }
}
