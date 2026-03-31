use std::fs;
use std::path::Path;
use std::process::Command;

pub fn compile_haskell_library(
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

pub fn copy_rts_library(rts_dir: &str, _rts_lib: &str, output_dir: &Path) {
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
