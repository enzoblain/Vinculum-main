use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

pub(super) fn find_ghc_version() -> Option<String> {
    let output = Command::new("ghc").arg("--numeric-version").output().ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

pub(super) fn find_rts_dir() -> String {
    if let Ok(dir) = env::var("HASKELL_RTS_DIR") {
        return dir;
    }

    if let Ok(output) = Command::new("ghc-pkg")
        .args(["field", "rts", "library-dirs"])
        .output()
        && output.status.success()
    {
        let out = String::from_utf8_lossy(&output.stdout);
        for line in out.lines() {
            let path_str = if line.contains("library-dirs:") {
                line.splitn(2, ':').nth(1).unwrap_or("").trim()
            } else {
                line.trim()
            };

            if path_str.is_empty() {
                continue;
            }

            let path = PathBuf::from(path_str);
            let resolved = path.canonicalize().unwrap_or(path);

            if resolved
                .read_dir()
                .ok()
                .and_then(|mut d| {
                    d.find(|e| {
                        e.as_ref()
                            .ok()
                            .map(|e| e.file_name().to_string_lossy().starts_with("libHSrts-ghc"))
                            .unwrap_or(false)
                    })
                })
                .is_some()
            {
                return resolved.to_string_lossy().to_string();
            }
        }
    }

    if let Ok(output) = Command::new("ghc").arg("--print-libdir").output()
        && output.status.success()
    {
        let libdir = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if let Ok(entries) = std::fs::read_dir(&libdir) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                if name.contains("-ghc-") && entry.path().is_dir() {
                    return entry.path().to_string_lossy().to_string();
                }
            }
        }

        let rts = PathBuf::from(&libdir).join("rts");
        if rts.exists() {
            return rts.to_string_lossy().to_string();
        }
    }

    panic!(
        "Could not find the GHC RTS directory.\n\
         Install GHC (via GHCup, Homebrew, or your package manager) \
         or set HASKELL_RTS_DIR manually."
    );
}

pub(super) fn find_rts_lib(rts_dir: &str) -> String {
    if let Ok(lib) = env::var("HASKELL_RTS_LIB") {
        return lib;
    }

    if let Some(version) = find_ghc_version() {
        return format!("HSrts-ghc{}", version);
    }

    if let Ok(entries) = std::fs::read_dir(rts_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();

            if name.starts_with("libHSrts-ghc") && name.ends_with(".a") {
                return name
                    .trim_start_matches("lib")
                    .trim_end_matches(".a")
                    .to_string();
            }
        }
    }

    panic!(
        "Could not detect the GHC RTS version.\n\
         Set HASKELL_RTS_LIB manually (e.g. HSrts-ghc9.6.7)."
    );
}

pub(crate) fn compile_haskell_library(
    haskell_dir: &Path,
    c_dir: &Path,
    user_functions_file: &Path,
    output_dir: &Path,
    lib_name: &str,
) {
    let ext = if cfg!(target_os = "windows") {
        "dll"
    } else if cfg!(target_os = "macos") {
        "dylib"
    } else {
        "so"
    };

    let lib_prefix = if cfg!(target_os = "windows") {
        ""
    } else {
        "lib"
    };

    let build_dir = output_dir.join("build");

    fs::create_dir_all(output_dir).expect("Failed to create Haskell output directory");
    fs::create_dir_all(&build_dir).expect("Failed to create Haskell intermediate build directory");

    let output_file = output_dir.join(format!("{lib_prefix}{lib_name}.{ext}"));

    let runtime = haskell_dir.join("Runtime.hs");
    let codec = haskell_dir.join("Codec.hs");
    let dispatch = haskell_dir.join("Dispatch.hs");
    let stubs_rts = c_dir.join("StubbsRTS.c");

    let status = Command::new("ghc")
        .args([
            "-dynamic",
            "-shared",
            "-outputdir",
            build_dir.to_str().expect("Invalid build directory path"),
            "-odir",
            build_dir.to_str().expect("Invalid object directory path"),
            "-hidir",
            build_dir
                .to_str()
                .expect("Invalid interface directory path"),
            "-stubdir",
            build_dir.to_str().expect("Invalid stub directory path"),
            "-o",
            output_file.to_str().expect("Invalid output library path"),
            runtime.to_str().expect("Invalid Runtime.hs path"),
            codec.to_str().expect("Invalid Codec.hs path"),
            dispatch.to_str().expect("Invalid Dispatch.hs path"),
            user_functions_file
                .to_str()
                .expect("Invalid UserFunctions.hs path"),
            stubs_rts.to_str().expect("Invalid StubbsRTS.c path"),
        ])
        .status()
        .expect("Failed to invoke GHC");

    if !status.success() {
        panic!("Haskell compilation failed");
    }
}

pub(crate) fn copy_rts_library(rts_dir: &str, _rts_lib: &str, output_dir: &Path) {
    let rts_path = Path::new(rts_dir);

    if !rts_path.exists() {
        println!("cargo:warning=RTS directory not found at: {}", rts_dir);
        return;
    }

    if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        let ext = if cfg!(target_os = "macos") {
            "dylib"
        } else {
            "so"
        };

        match fs::read_dir(rts_path) {
            Ok(entries) => {
                let mut copied_count = 0;

                for entry in entries.flatten() {
                    let path = entry.path();

                    if let Some(file_str) = path.file_name().and_then(|f| f.to_str())
                        && file_str.starts_with("libHS")
                        && file_str.ends_with(ext)
                    {
                        let dest = output_dir.join(file_str);

                        match fs::copy(&path, &dest) {
                            Ok(_) => copied_count += 1,
                            Err(e) => println!("cargo:warning=Failed to copy {}: {}", file_str, e),
                        }
                    }
                }

                if copied_count > 0 {
                    println!(
                        "cargo:warning=Copied {} Haskell libraries to {}",
                        copied_count,
                        output_dir.display()
                    );
                }
            }
            Err(e) => {
                println!(
                    "cargo:warning=Failed to read RTS directory {}: {}",
                    rts_dir, e
                );
            }
        }
    }
}
