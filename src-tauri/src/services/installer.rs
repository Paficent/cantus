use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::errors::{AppError, AppResult};
use crate::services::mods;

enum ArchiveKind {
    Zip,
    SevenZ,
    Rar,
}

fn detect_kind(path: &Path) -> AppResult<ArchiveKind> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();
    match ext.as_str() {
        "zip" => Ok(ArchiveKind::Zip),
        "7z" => Ok(ArchiveKind::SevenZ),
        "rar" => Ok(ArchiveKind::Rar),
        _ => Err(AppError::from(format!(
            "Unsupported archive format: .{ext}"
        ))),
    }
}

fn staging_dir() -> AppResult<PathBuf> {
    let base = std::env::temp_dir().join("cantus").join("staging");
    std::fs::create_dir_all(&base)?;
    Ok(base)
}

fn unique_staging_path() -> AppResult<PathBuf> {
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let path = staging_dir()?.join(format!("mod_{id}"));
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

fn extract(archive_path: &Path, dest: &Path) -> AppResult<()> {
    match detect_kind(archive_path)? {
        ArchiveKind::Zip => extract_zip(archive_path, dest),
        ArchiveKind::SevenZ => extract_7z(archive_path, dest),
        ArchiveKind::Rar => extract_rar(archive_path, dest),
    }
}

fn extract_zip(archive_path: &Path, dest: &Path) -> AppResult<()> {
    let file = std::fs::File::open(archive_path)?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| AppError::from(format!("Failed to open zip: {e}")))?;
    archive
        .extract(dest)
        .map_err(|e| AppError::from(format!("Failed to extract zip: {e}")))?;
    Ok(())
}

fn extract_7z(archive_path: &Path, dest: &Path) -> AppResult<()> {
    sevenz_rust::decompress_file(archive_path, dest)
        .map_err(|e| AppError::from(format!("Failed to extract 7z: {e}")))?;
    Ok(())
}

fn extract_rar(archive_path: &Path, dest: &Path) -> AppResult<()> {
    let mut archive = unrar::Archive::new(archive_path)
        .open_for_processing()
        .map_err(|e| AppError::from(format!("Failed to open rar: {e}")))?;

    while let Some(header) = archive
        .read_header()
        .map_err(|e| AppError::from(format!("Failed to read rar header: {e}")))?
    {
        archive = header
            .extract_with_base(dest)
            .map_err(|e| AppError::from(format!("Failed to extract rar entry: {e}")))?;
    }

    Ok(())
}

fn find_mod_root(extracted: &Path) -> PathBuf {
    if let Some(manifest) = find_manifest_recursive(extracted) {
        manifest
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| extracted.to_path_buf())
    } else {
        let entries: Vec<_> = std::fs::read_dir(extracted)
            .into_iter()
            .flat_map(|rd| rd.flatten().collect::<Vec<_>>())
            .collect();

        if entries.len() == 1 && entries[0].path().is_dir() {
            entries[0].path()
        } else {
            extracted.to_path_buf()
        }
    }
}

fn find_manifest_recursive(dir: &Path) -> Option<PathBuf> {
    let manifest = dir.join("manifest.json");
    if manifest.exists() {
        return Some(manifest);
    }
    let mut entries: Vec<_> = std::fs::read_dir(dir).ok()?.flatten().collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            if let Some(found) = find_manifest_recursive(&path) {
                return Some(found);
            }
        }
    }
    None
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> AppResult<()> {
    std::fs::create_dir_all(dest)?;
    for entry in std::fs::read_dir(src)?.flatten() {
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dest_path)?;
        } else {
            std::fs::copy(&src_path, &dest_path)?;
        }
    }
    Ok(())
}

fn collect_files_recursive(dir: &Path) -> AppResult<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_files_inner(dir, &mut files)?;
    Ok(files)
}

fn collect_files_inner(dir: &Path, out: &mut Vec<PathBuf>) -> AppResult<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)?.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files_inner(&path, out)?;
        } else {
            out.push(path);
        }
    }
    Ok(())
}

fn build_game_file_index(data_dir: &Path) -> AppResult<HashMap<String, Vec<PathBuf>>> {
    let mut index: HashMap<String, Vec<PathBuf>> = HashMap::new();
    if !data_dir.is_dir() {
        return Ok(index);
    }

    let files = collect_files_recursive(data_dir)?;
    for file in files {
        let relative = file
            .strip_prefix(data_dir)
            .unwrap_or(&file)
            .to_path_buf();
        let filename = file
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();
        if !filename.is_empty() {
            index.entry(filename).or_default().push(relative);
        }
    }

    Ok(index)
}

fn mod_name_from_archive(archive_path: &Path) -> String {
    archive_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown-mod")
        .to_string()
}

fn install_native_jeode_mod(root: &Path, mods_dir: &Path) -> AppResult<String> {
    let content = std::fs::read_to_string(root.join("manifest.json"))?;
    let manifest: mods::Manifest = serde_json::from_str(&content)
        .map_err(|e| AppError::from(format!("Invalid manifest.json: {e}")))?;
    let id = mods::sanitize_id(&manifest.id);
    let name = manifest.name.clone();
    let dest = mods_dir.join(&id);

    if dest.exists() {
        std::fs::remove_dir_all(&dest)?;
    }

    copy_dir_recursive(root, &dest)?;
    Ok(name)
}

/// true = no 'data/' but subdirectories map to actual data subdirectories (gfx, etc.)
fn looks_like_data_contents(mod_root: &Path, game_dir: &Path) -> bool {
    let game_data = game_dir.join("data");
    if !game_data.is_dir() {
        return false;
    }

    let mod_dirs: Vec<_> = std::fs::read_dir(mod_root)
        .into_iter()
        .flat_map(|rd| rd.flatten().collect::<Vec<_>>())
        .filter(|e| e.path().is_dir())
        .collect();

    if mod_dirs.is_empty() {
        return false;
    }

    let matching = mod_dirs
        .iter()
        .filter(|e| game_data.join(e.file_name()).is_dir())
        .count();

    // If at least half of the mod's directories exist inside data
    matching > 0 && matching * 2 >= mod_dirs.len()
}

fn install_rebuilt_mod(
    archive_path: &Path,
    root: &Path,
    game_dir: &Path,
    mods_dir: &Path,
) -> AppResult<String> {
    let raw_name = mod_name_from_archive(archive_path);
    let mod_id = mods::sanitize_id(&raw_name);
    let mod_dir = mods_dir.join(&mod_id);

    if mod_dir.exists() {
        std::fs::remove_dir_all(&mod_dir)?;
    }
    std::fs::create_dir_all(&mod_dir)?;

    if root.join("data").is_dir() {
        copy_dir_recursive(root, &mod_dir)?;
    } else if looks_like_data_contents(root, game_dir) {
        let dest_data = mod_dir.join("data");
        copy_dir_recursive(root, &dest_data)?;
    } else {
        let game_data = game_dir.join("data");
        let index = build_game_file_index(&game_data)?;
        let archive_files = collect_files_recursive(root)?;
        let mut matched_any = false;

        for file_path in &archive_files {
            let filename = file_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_lowercase();

            if let Some(game_relatives) = index.get(&filename) {
                if let Some(game_rel) = game_relatives.first() {
                    let dest = mod_dir.join("data").join(game_rel);
                    if let Some(parent) = dest.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    std::fs::copy(file_path, &dest)?;
                    matched_any = true;
                    continue;
                }
            }

            let relative = file_path
                .strip_prefix(root)
                .unwrap_or(file_path)
                .to_path_buf();
            let dest = mod_dir.join(&relative);
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(file_path, &dest)?;
        }

        if !matched_any {
            log::info!("No game data matches found for {raw_name}, installed files as-is");
        }
    }

    let manifest = mods::Manifest {
        id: mod_id,
        name: raw_name.clone(),
        assets: mods::ManifestAssets {
            auto_override: true,
            ..Default::default()
        },
        ..mods::default_manifest()
    };
    mods::write_manifest(&mod_dir.join("manifest.json"), &manifest)?;

    Ok(raw_name)
}

pub fn install(archive_path: &Path, game_dir: &Path) -> AppResult<String> {
    let staging = unique_staging_path()?;
    let result = install_inner(archive_path, game_dir, &staging);
    let _ = std::fs::remove_dir_all(&staging);
    result
}

fn install_inner(archive_path: &Path, game_dir: &Path, staging: &Path) -> AppResult<String> {
    extract(archive_path, staging)?;

    let root = find_mod_root(staging);
    let mods_dir = game_dir.join("mods");
    std::fs::create_dir_all(&mods_dir)?;

    if root.join("manifest.json").exists() {
        install_native_jeode_mod(&root, &mods_dir)
    } else {
        install_rebuilt_mod(archive_path, &root, game_dir, &mods_dir)
    }
}
