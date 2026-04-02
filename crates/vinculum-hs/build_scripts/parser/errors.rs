use std::error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) enum ParseError {
    ReadFile {
        path: PathBuf,
        source: io::Error,
    },
    EmptySignature,
    InvalidFunctionName {
        name: String,
        signature: String,
    },
    ReservedRustKeyword {
        name: String,
    },
    UnsupportedHaskellType(String),
    MissingHaskellTypeAnnotation {
        signature: String,
    },
    MissingReturnHaskellType {
        signature: String,
    },
    ArgumentCountMismatch {
        expected: usize,
        found: usize,
        signature: String,
    },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ParseError::ReadFile { path, source } => {
                write!(f, "failed to read file '{}': {}", path.display(), source)
            }
            ParseError::EmptySignature => {
                write!(f, "empty function signature")
            }
            ParseError::InvalidFunctionName { name, signature } => {
                write!(
                    f,
                    "invalid function name `{}` in signature `{}` (not a valid Rust identifier)",
                    name, signature
                )
            }
            ParseError::ReservedRustKeyword { name } => {
                write!(f, "name `{}` is a reserved Rust keyword", name)
            }
            ParseError::UnsupportedHaskellType(t) => {
                write!(f, "unsupported type `{}`", t)
            }
            ParseError::MissingHaskellTypeAnnotation { signature } => {
                write!(
                    f,
                    "missing type annotation in signature `{}` (expected `::`)",
                    signature
                )
            }
            ParseError::MissingReturnHaskellType { signature } => {
                write!(f, "missing return type in signature `{}`", signature)
            }
            ParseError::ArgumentCountMismatch {
                expected,
                found,
                signature,
            } => {
                write!(
                    f,
                    "argument count mismatch in `{}`: expected {}, found {}",
                    signature, expected, found
                )
            }
        }
    }
}

impl error::Error for ParseError {}
