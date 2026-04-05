use std::path::PathBuf;

use cargo_metadata::MetadataCommand;

pub struct HaskellConfig {
    pub cabal_file: PathBuf,
    pub exports_dir: PathBuf,
    pub foreign_library: String,
}

const FALLBACK_CABAL_FILE: &str = "haskell_fallback/haskell.cabal";
const FALLBACK_EXPORTS_DIR: &str = "haskell_fallback/app/exports";
const FALLBACK_FOREIGN_LIBRARY: &str = "lib";

pub(crate) fn load_haskell_config() -> Result<HaskellConfig, String> {
    let metadata = MetadataCommand::new()
        .exec()
        .map_err(|e| format!("failed to read cargo metadata: {e}"))?;

    let package = metadata
        .root_package()
        .ok_or_else(|| "no root package found".to_string())?;

    let manifest_dir = PathBuf::from(&package.manifest_path)
        .parent()
        .ok_or_else(|| "failed to get manifest directory".to_string())?
        .to_path_buf();

    let cabal_meta = package.metadata.get("cabal_file").and_then(|v| v.as_str());
    let exports_meta = package.metadata.get("exports_dir").and_then(|v| v.as_str());
    let foreign_meta = package
        .metadata
        .get("foreign_library")
        .and_then(|v| v.as_str());

    let all_defined = cabal_meta.is_some() && exports_meta.is_some() && foreign_meta.is_some();
    let none_defined = cabal_meta.is_none() && exports_meta.is_none() && foreign_meta.is_none();

    if !all_defined && !none_defined {
        return Err(
            "invalid config: define ALL of `cabal_file`, `exports_dir`, `foreign_library`, \
             or NONE to use fallbacks"
                .to_string(),
        );
    }

    if all_defined {
        let cabal_file = manifest_dir.join(cabal_meta.unwrap());
        let exports_dir = manifest_dir.join(exports_meta.unwrap());
        let foreign_library = foreign_meta.unwrap().to_string();

        if !cabal_file.exists() {
            return Err(format!(
                "cabal file does not exist: {}",
                cabal_file.display()
            ));
        }

        if !exports_dir.exists() {
            return Err(format!(
                "exports directory does not exist: {}",
                exports_dir.display()
            ));
        }

        return Ok(HaskellConfig {
            cabal_file,
            exports_dir,
            foreign_library,
        });
    }

    let cabal_file = manifest_dir.join(FALLBACK_CABAL_FILE);
    let exports_dir = manifest_dir.join(FALLBACK_EXPORTS_DIR);
    let foreign_library = FALLBACK_FOREIGN_LIBRARY.to_string();

    if !cabal_file.exists() {
        return Err(format!(
            "fallback cabal file does not exist: {}",
            cabal_file.display()
        ));
    }

    if !exports_dir.exists() {
        return Err(format!(
            "fallback exports directory does not exist: {}",
            exports_dir.display()
        ));
    }

    println!(
        "cargo:warning=[vinculum] Using fallback Haskell config:\n  cabal_file: {}\n  exports_dir: {}\n  foreign_library: {}",
        cabal_file.display(),
        exports_dir.display(),
        foreign_library
    );

    Ok(HaskellConfig {
        cabal_file,
        exports_dir,
        foreign_library,
    })
}
