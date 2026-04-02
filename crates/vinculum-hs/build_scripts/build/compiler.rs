use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

pub(crate) fn find_ghc_version() -> Option<String> {
    let output = Command::new("ghc").arg("--numeric-version").output().ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

pub(crate) fn find_rts_dir() -> String {
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
                line.split_once(':').map(|x| x.1).unwrap_or("").trim()
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

        if let Ok(entries) = fs::read_dir(&libdir) {
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

pub(crate) fn find_rts_lib(rts_dir: &str) -> String {
    if let Ok(lib) = env::var("HASKELL_RTS_LIB") {
        return lib;
    }

    if let Some(version) = find_ghc_version() {
        return format!("HSrts-ghc{}", version);
    }

    if let Ok(entries) = fs::read_dir(rts_dir) {
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
    ffi_dir: &Path,
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

    let runtime = ffi_dir.join("lib/Runtime.hs");
    let codec = ffi_dir.join("lib/Codec.hs");
    let dispatch = ffi_dir.join("generated/Dispatch.hs");

    let stubs_rts_src = ffi_dir.join("lib/StubbsRTS.c");
    let stubs_rts = build_dir.join("StubbsRTS.c");
    fs::copy(&stubs_rts_src, &stubs_rts).expect("Failed to copy StubbsRTS.c to build directory");

    let include_path = format!(
        "-i{}",
        output_dir.to_str().expect("Invalid output directory path")
    );

    let status = Command::new("ghc")
        .arg("-dynamic")
        .arg("-shared")
        .arg(&include_path)
        .arg("-outputdir")
        .arg(build_dir.to_str().expect("Invalid build directory path"))
        .arg("-odir")
        .arg(build_dir.to_str().expect("Invalid object directory path"))
        .arg("-hidir")
        .arg(
            build_dir
                .to_str()
                .expect("Invalid interface directory path"),
        )
        .arg("-stubdir")
        .arg(build_dir.to_str().expect("Invalid stub directory path"))
        .arg("-o")
        .arg(output_file.to_str().expect("Invalid output library path"))
        .arg(runtime.to_str().expect("Invalid Runtime.hs path"))
        .arg(codec.to_str().expect("Invalid Codec.hs path"))
        .arg(dispatch.to_str().expect("Invalid Dispatch.hs path"))
        .arg(
            user_functions_file
                .to_str()
                .expect("Invalid UserFunctions.hs path"),
        )
        .arg(stubs_rts.to_str().expect("Invalid StubbsRTS.c path"))
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
                for entry in entries.flatten() {
                    let path = entry.path();

                    if let Some(file_str) = path.file_name().and_then(|f| f.to_str())
                        && file_str.starts_with("libHS")
                        && file_str.ends_with(ext)
                    {
                        let dest = output_dir.join(file_str);

                        if let Err(e) = fs::copy(&path, &dest) {
                            println!("cargo:warning=Failed to copy {}: {}", file_str, e);
                        }
                    }
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
