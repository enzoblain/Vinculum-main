use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RustGeneratorError {
    #[error("Failed to get CARGO_MANIFEST_DIR environment variable")]
    ManifestDirNotFound,

    #[error("IO error during file operations: {0}")]
    Io(#[from] io::Error),

    #[error("Failed to read or write file: {0}")]
    FileOperation(String),

    #[error("Failed to add vinculum module to source file")]
    AddVinculumModFailed,

    #[error("Failed to generate files: {0}")]
    GenerationFailed(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Invalid file structure: {0}")]
    InvalidFileStructure(String),
}
