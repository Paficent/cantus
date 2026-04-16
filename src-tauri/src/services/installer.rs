use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::errors::{AppError, AppResult};
use crate::services::mods;

enum ArchiveKind {
    Zip,
    SevenZ,
    Rar,
}

impl ArchiveKind {
    fn from_path(path: &Path) -> AppResult<Self> {
        match path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .as_deref()
        {
            Some("zip") => Ok(Self::Zip),
            Some("7z") => Ok(Self::SevenZ),
            Some("rar") => Ok(Self::Rar),
            Some(ext) => Err(format!("Unsupported archive format: .{ext}").into()),
            None => Err("No file extension ???".into()),
        }
    }
}

fn staging_dir() -> AppResult<PathBuf> {
    let base = std::env::temp_dir().join("cantus/staging");
    std::fs::create_dir_all(&base)?;
    Ok(base)
}

fn unique_staging_path() -> AppResult<PathBuf> {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);

    let path = staging_dir()?.join(format!("mod_{ts}"));
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

fn extract(archive: &Path, dest: &Path) -> AppResult<()> {
    match ArchiveKind::from_path(archive)? {
        ArchiveKind::Zip => extract_zip(archive, dest),
        ArchiveKind::SevenZ => extract_7z(archive, dest),
        ArchiveKind::Rar => extract_rar(archive, dest),
    }
}

fn extract_zip(path: &Path, dest: &Path) -> AppResult<()> {
    let file = std::fs::File::open(path)?;
    let mut ar = zip::ZipArchive::new(file)
        .map_err(|e| AppError::from(format!("Failed to open zip: {e}")))?;
    ar.extract(dest)
        .map_err(|e| AppError::from(format!("Failed to extract zip: {e}")))?;
    Ok(())
}

fn extract_7z(path: &Path, dest: &Path) -> AppResult<()> {
    sevenz_rust::decompress_file(path, dest)
        .map_err(|e| AppError::from(format!("7z extraction failed: {e}")))?;
    Ok(())
}

fn extract_rar(path: &Path, dest: &Path) -> AppResult<()> {
    let mut ar = unrar::Archive::new(path)
        .open_for_processing()
        .map_err(|e| AppError::from(format!("Failed to open rar: {e}")))?;

    while let Some(header) = ar
        .read_header()
        .map_err(|e| AppError::from(format!("Corrupt rar header: {e}")))?
    {
        ar = header
            .extract_with_base(dest)
            .map_err(|e| AppError::from(format!("Rar extraction failed: {e}")))?;
    }
    Ok(())
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> AppResult<()> {
    std::fs::create_dir_all(dest)?;
    for entry in std::fs::read_dir(src)?.flatten() {
        let target = dest.join(entry.file_name());
        if entry.path().is_dir() {
            copy_dir_recursive(&entry.path(), &target)?;
            continue;
        }
        std::fs::copy(entry.path(), target)?;
    }
    Ok(())
}

fn copy_file_creating_parents(src: &Path, dest: &Path) -> AppResult<()> {
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::copy(src, dest)?;
    Ok(())
}

fn collect_files(dir: &Path) -> AppResult<Vec<PathBuf>> {
    let mut out = Vec::new();
    collect_files_walk(dir, &mut out)?;
    Ok(out)
}

fn collect_files_walk(dir: &Path, out: &mut Vec<PathBuf>) -> AppResult<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)?.flatten() {
        let p = entry.path();
        if p.is_dir() {
            collect_files_walk(&p, out)?;
            continue;
        }
        out.push(p);
    }
    Ok(())
}

fn build_file_index(data_dir: &Path) -> AppResult<HashMap<String, Vec<PathBuf>>> {
    let mut index: HashMap<String, Vec<PathBuf>> = HashMap::new();
    if !data_dir.is_dir() {
        return Ok(index);
    }
    for file in collect_files(data_dir)? {
        let Some(name) = file.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let rel = file.strip_prefix(data_dir).unwrap_or(&file).to_path_buf();
        index.entry(name.to_lowercase()).or_default().push(rel);
    }
    Ok(index)
}

fn find_mod_root(extracted: &Path) -> PathBuf {
    if let Some(manifest) = find_manifest(extracted) {
        return manifest.parent().unwrap_or(extracted).to_path_buf();
    }

    let entries: Vec<_> = std::fs::read_dir(extracted)
        .ok()
        .into_iter()
        .flatten()
        .flatten()
        .collect();

    match entries.as_slice() {
        [only] if only.path().is_dir() => only.path(),
        _ => extracted.to_path_buf(),
    }
}

fn find_manifest(dir: &Path) -> Option<PathBuf> {
    let candidate = dir.join("manifest.json");
    if candidate.exists() {
        return Some(candidate);
    }
    let mut children: Vec<_> = std::fs::read_dir(dir).ok()?.flatten().collect();
    children.sort_by_key(|e| e.file_name());

    children
        .iter()
        .filter_map(|e| {
            let p = e.path();
            if p.is_dir() {
                find_manifest(&p)
            } else {
                None
            }
        })
        .next()
}

fn mod_name_from_archive(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown-mod")
        .to_string()
}

// native as in already a jeode mod not DLL mods
fn install_native(root: &Path, mods_dir: &Path) -> AppResult<String> {
    let raw = std::fs::read_to_string(root.join("manifest.json"))?;
    let manifest: mods::Manifest = serde_json::from_str(&raw)
        .map_err(|e| AppError::from(format!("Invalid manifest.json: {e}")))?;

    let id = mods::sanitize_id(&manifest.id);
    let dest = mods_dir.join(&id);

    if dest.exists() {
        std::fs::remove_dir_all(&dest)?;
    }
    copy_dir_recursive(root, &dest)?;

    Ok(manifest.name)
}

// no data directory but has gfx etc
fn looks_like_bare_data(mod_root: &Path, game_dir: &Path) -> bool {
    let game_data = game_dir.join("data");
    if !game_data.is_dir() {
        return false;
    }

    let subdirs: Vec<_> = std::fs::read_dir(mod_root)
        .ok()
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().is_dir())
        .collect();

    if subdirs.is_empty() {
        return false;
    }

    let hits = subdirs
        .iter()
        .filter(|e| game_data.join(e.file_name()).is_dir())
        .count();

    hits > 0 && hits * 2 >= subdirs.len()
}

fn install_rebuilt(
    archive_path: &Path,
    root: &Path,
    game_dir: &Path,
    mods_dir: &Path,
) -> AppResult<String> {
    let name = mod_name_from_archive(archive_path);
    let id = mods::sanitize_id(&name);
    let mod_dir = mods_dir.join(&id);

    if mod_dir.exists() {
        std::fs::remove_dir_all(&mod_dir)?;
    }
    std::fs::create_dir_all(&mod_dir)?;

    if root.join("data").is_dir() {
        copy_dir_recursive(root, &mod_dir)?;
    } else if looks_like_bare_data(root, game_dir) {
        copy_dir_recursive(root, &mod_dir.join("data"))?;
    } else {
        place_files_by_name(root, game_dir, &mod_dir)?;
    }

    let manifest = mods::Manifest {
        id,
        name: name.clone(),
        assets: mods::ManifestAssets {
            auto_override: true,
            ..Default::default()
        },
        ..mods::default_manifest()
    };
    mods::write_manifest(&mod_dir.join("manifest.json"), &manifest)?;

    Ok(name)
}

fn place_files_by_name(root: &Path, game_dir: &Path, mod_dir: &Path) -> AppResult<()> {
    let game_data = game_dir.join("data");
    let index = build_file_index(&game_data)?;
    let mut matched_any = false;

    for file in &collect_files(root)? {
        let filename = file
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();

        if let Some(rel) = index.get(&filename).and_then(|v| v.first()) {
            copy_file_creating_parents(file, &mod_dir.join("data").join(rel))?;
            matched_any = true;
            continue;
        }

        let rel = file.strip_prefix(root).unwrap_or(file);
        copy_file_creating_parents(file, &mod_dir.join(rel))?;
    }

    if matched_any {
        return Ok(());
    }

    log::info!(
        "No data matches for {}, installed as normal",
        mod_name_from_archive(&root.to_path_buf().join("_"))
    );
    Ok(())
}

fn do_install(archive_path: &Path, game_dir: &Path, staging: &Path) -> AppResult<String> {
    extract(archive_path, staging)?;
    let root = find_mod_root(staging);
    let mods_dir = game_dir.join("mods");
    std::fs::create_dir_all(&mods_dir)?;

    if root.join("manifest.json").exists() {
        return install_native(&root, &mods_dir);
    }
    install_rebuilt(archive_path, &root, game_dir, &mods_dir)
}

pub fn install(archive_path: &Path, game_dir: &Path) -> AppResult<String> {
    let staging = unique_staging_path()?;
    let result = do_install(archive_path, game_dir, &staging);
    let _ = std::fs::remove_dir_all(&staging);
    result
}
