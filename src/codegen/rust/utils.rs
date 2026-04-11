use super::errors::RustGeneratorError;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub(crate) fn get_cargo_manifest_path() -> Result<PathBuf, RustGeneratorError> {
    std::env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .map_err(|_| RustGeneratorError::ManifestDirNotFound)
}

pub(crate) fn add_vinculum_mod(src_path: &Path) -> Result<(), RustGeneratorError> {
    let lib_path = src_path.join("lib.rs");
    let main_path = src_path.join("main.rs");

    let (target, is_lib) = if lib_path.exists() {
        (lib_path, true)
    } else if main_path.exists() {
        (main_path, false)
    } else {
        return Err(RustGeneratorError::InvalidFileStructure(
            "Neither lib.rs nor main.rs found".to_string(),
        ));
    };

    let mut content = fs::read_to_string(&target)
        .map_err(|e| RustGeneratorError::FileOperation(e.to_string()))?;

    if content.contains("mod vinculum") {
        return Ok(());
    }

    let line = if is_lib {
        "pub(crate) mod vinculum;\n"
    } else {
        "mod vinculum;\n"
    };

    if is_lib {
        if !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str(line);
    } else {
        content = format!("{}\n{}", line, content);
    }

    let mut file =
        fs::File::create(&target).map_err(|e| RustGeneratorError::FileOperation(e.to_string()))?;
    file.write_all(content.as_bytes())
        .map_err(|e| RustGeneratorError::FileOperation(e.to_string()))?;

    Ok(())
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
