use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

pub(crate) fn find_cabal() -> Result<PathBuf, String> {
    let exe = if cfg!(windows) { "cabal.exe" } else { "cabal" };
    let path = env::var_os("PATH").ok_or("PATH environment variable not set")?;

    env::split_paths(&path)
        .map(|dir| dir.join(exe))
        .find(|path| path.is_file())
        .ok_or("cabal executable not found in PATH".to_string())
}

pub(crate) fn build_haskell_dll(
    cabal_path: &Path,
    cabal_file: &Path,
    foreign_library: &str,
) -> Result<(), String> {
    let project_dir = cabal_file
        .parent()
        .ok_or_else(|| format!("invalid cabal file path: {}", cabal_file.display()))?;

    let target = format!("flib:{foreign_library}");

    let output = Command::new(cabal_path)
        .args(["build", &target])
        .current_dir(project_dir)
        .output()
        .map_err(|e| format!("failed to run cabal build: {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "`cabal build {target}` failed in {}\nstdout:\n{}\nstderr:\n{}",
            project_dir.display(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        ));
    }

    let ext = if cfg!(target_os = "macos") {
        "dylib"
    } else if cfg!(target_os = "linux") {
        "so"
    } else if cfg!(target_os = "windows") {
        "dll"
    } else {
        return Err("unsupported target OS for dynamic libraries".to_string());
    };

    let file_name = format!("lib{foreign_library}.{ext}");
    let dist_dir = project_dir.join("dist-newstyle");

    let src = find_library_recursive(&dist_dir, &file_name)
        .ok_or_else(|| format!("library {file_name} not found in {}", dist_dir.display()))?;

    let target_dir = env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .or_else(|_| {
            env::var("CARGO_MANIFEST_DIR").map(|dir| {
                PathBuf::from(dir)
                    .parent()
                    .and_then(|p| p.parent())
                    .unwrap()
                    .join("target")
            })
        })
        .map_err(|_| "failed to resolve target directory".to_string())?
        .join("haskell");

    fs::create_dir_all(&target_dir)
        .map_err(|e| format!("failed to create {}: {e}", target_dir.display()))?;

    fs::copy(&src, target_dir.join(&file_name))
        .map_err(|e| format!("failed to copy {}: {e}", src.display()))?;

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
