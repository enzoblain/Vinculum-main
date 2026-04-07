use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use super::errors::CompilerError;

pub(crate) fn find_cabal() -> Result<PathBuf, CompilerError> {
    let exe = if cfg!(windows) { "cabal.exe" } else { "cabal" };
    let path = env::var_os("PATH").ok_or(CompilerError::PathNotSet)?;

    env::split_paths(&path)
        .map(|dir| dir.join(exe))
        .find(|path| path.is_file())
        .ok_or(CompilerError::CabalNotFound)
}

pub(crate) fn build_haskell_dll(
    cabal_path: &Path,
    cabal_file: &Path,
    foreign_library: &str,
) -> Result<(), CompilerError> {
    let project_dir = cabal_file
        .parent()
        .ok_or_else(|| CompilerError::InvalidCabalPath {
            path: cabal_file.to_path_buf(),
        })?;

    let target = format!("flib:{foreign_library}");

    let output = Command::new(cabal_path)
        .args(["build", &target])
        .current_dir(project_dir)
        .output()
        .map_err(|e| CompilerError::CabalBuildFailed {
            target: target.clone(),
            reason: e.to_string(),
        })?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(CompilerError::CabalBuildFailed {
            target,
            reason: format!(
                "in {}\nstdout:\n{}\nstderr:\n{}",
                project_dir.display(),
                stdout,
                stderr,
            ),
        });
    }

    let ext = if cfg!(target_os = "macos") {
        "dylib"
    } else if cfg!(target_os = "linux") {
        "so"
    } else if cfg!(target_os = "windows") {
        "dll"
    } else {
        return Err(CompilerError::UnsupportedOS);
    };

    let file_name = format!("lib{foreign_library}.{ext}");
    let dist_dir = project_dir.join("dist-newstyle");

    let src = find_library_recursive(&dist_dir, &file_name).ok_or_else(|| {
        CompilerError::LibraryNotFound {
            library: file_name.clone(),
            path: dist_dir.clone(),
        }
    })?;

    let target_dir = env::var("CARGO_MANIFEST_DIR")
        .map(|dir| PathBuf::from(dir).join("target").join("haskell"))
        .unwrap_or_else(|_| Path::new("target/haskell").into());

    fs::create_dir_all(&target_dir).map_err(|e| CompilerError::DirectoryCreationFailed {
        path: target_dir.clone(),
        reason: e.to_string(),
    })?;

    fs::copy(&src, target_dir.join(&file_name)).map_err(|e| CompilerError::FileCopyFailed {
        src,
        dst: target_dir.join(&file_name),
        reason: e.to_string(),
    })?;

    Ok(())
}

fn find_library_recursive(dir: &Path, expected: &str) -> Option<PathBuf> {
    for entry in fs::read_dir(dir).ok()?.flatten() {
        let path = entry.path();

        if path.is_dir() {
            if let Some(found) = find_library_recursive(&path, expected) {
                return Some(found);
            }
        } else if path.file_name()?.to_str()? == expected {
            return Some(path);
        }
    }

    None
}
