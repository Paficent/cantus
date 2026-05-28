use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::Serialize;
use walkdir::WalkDir;

use crate::errors::{AppResult, Context};
use crate::services::mods;

pub struct ModMetadata {
    pub id: String,
    pub name: String,
    pub author: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct InstallResult {
    pub installed: Vec<String>,
    pub total: usize,
    pub error: Option<String>,
}

enum Archive {
    Zip,
    SevenZ,
    Rar,
}

impl Archive {
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

    fn extract(self, archive: &Path, dest: &Path) -> AppResult<()> {
        match self {
            Self::Zip => {
                let file = std::fs::File::open(archive)?;
                let mut ar = zip::ZipArchive::new(file).context("failed to open zip")?;
                ar.extract(dest).context("failed to extract zip")?;
            }
            Self::SevenZ => {
                sevenz_rust::decompress_file(archive, dest).context("7z extraction failed")?;
            }
            Self::Rar => {
                let mut ar = unrar::Archive::new(archive)
                    .open_for_processing()
                    .context("failed to open rar")?;
                while let Some(header) = ar.read_header().context("corrupt rar header")? {
                    ar = header
                        .extract_with_base(dest)
                        .context("rar extraction failed")?;
                }
            }
        }
        Ok(())
    }
}

struct StagingDir(PathBuf);

impl StagingDir {
    fn new() -> AppResult<Self> {
        let base = std::env::temp_dir().join("cantus/staging");
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let path = base.join(format!("mod_{ts}"));
        std::fs::create_dir_all(&path)?;
        Ok(Self(path))
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for StagingDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> AppResult<()> {
    for entry in WalkDir::new(src) {
        let entry = entry.context("walk failed")?;
        let rel = entry.path().strip_prefix(src).unwrap();
        let target = dest.join(rel);

        if entry.file_type().is_dir() {
            std::fs::create_dir_all(&target)?;
        } else {
            std::fs::copy(entry.path(), &target)?;
        }
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

fn convert_to_avif(src: &Path, dest: &Path) -> AppResult<()> {
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let img = image::open(src).context("failed to read image")?;
    let mut out = std::fs::File::create(dest)?;
    img.write_with_encoder(image::codecs::avif::AvifEncoder::new_with_speed_quality(
        &mut out, 4, 90,
    ))
    .context("avif conversion failed")?;
    Ok(())
}

fn collect_files(dir: &Path) -> AppResult<Vec<PathBuf>> {
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    Ok(WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .collect())
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

fn find_mod_roots(extracted: &Path) -> AppResult<Vec<PathBuf>> {
    let direct: Vec<PathBuf> = WalkDir::new(extracted)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir() && e.path().join("manifest.json").exists())
        .map(|e| e.into_path())
        .collect();

    if direct.len() > 1 {
        return Ok(direct);
    }

    if let Some(manifest) = find_manifest(extracted) {
        return Ok(vec![manifest.parent().unwrap_or(extracted).to_path_buf()]);
    }

    let top: Vec<PathBuf> = WalkDir::new(extracted)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
        .map(|e| e.into_path())
        .collect();

    Ok(vec![match top.as_slice() {
        [only] => only.clone(),
        _ => extracted.to_path_buf(),
    }])
}

fn find_manifest(dir: &Path) -> Option<PathBuf> {
    WalkDir::new(dir)
        .sort_by_file_name()
        .into_iter()
        .filter_map(Result::ok)
        .map(|e| e.into_path())
        .find(|p| p.file_name().and_then(|n| n.to_str()) == Some("manifest.json"))
}

fn mod_name_from_archive(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown-mod")
        .to_string()
}

// no data directory but has gfx etc
fn looks_like_bare_data(mod_root: &Path, game_dir: &Path) -> bool {
    let game_data = game_dir.join("data");
    if !game_data.is_dir() {
        return false;
    }

    let subdirs: Vec<_> = WalkDir::new(mod_root)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
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

fn place_files_by_name(
    root: &Path,
    game_dir: &Path,
    mod_dir: &Path,
    convert: bool,
) -> AppResult<()> {
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

        if filename.ends_with(".png") || filename.ends_with(".jpg") || filename.ends_with(".jpeg") {
            let stem = Path::new(&filename)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            let avif_name = format!("{stem}.avif");
            if let Some(rel) = index.get(&avif_name).and_then(|v| v.first()) {
                if convert {
                    convert_to_avif(file, &mod_dir.join("data").join(rel))?;
                } else {
                    let dir = match rel.parent() {
                        Some(parent) => mod_dir.join("data").join(parent),
                        None => mod_dir.join("data"),
                    };
                    copy_file_creating_parents(
                        file,
                        &dir.join(file.file_name().unwrap_or_default()),
                    )?;
                }
                matched_any = true;
                continue;
            }
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

fn install_rebuilt(
    archive_path: &Path,
    root: &Path,
    game_dir: &Path,
    mods_dir: &Path,
    metadata: Option<&ModMetadata>,
    convert: bool,
) -> AppResult<String> {
    let name = metadata
        .map(|m| m.name.clone())
        .unwrap_or_else(|| mod_name_from_archive(archive_path));
    let id = metadata
        .map(|m| mods::sanitize_id(&m.id))
        .unwrap_or_else(|| mods::sanitize_id(&name));
    let author = metadata
        .map(|m| m.author.clone())
        .unwrap_or_else(|| "Unknown".into());
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
        place_files_by_name(root, game_dir, &mod_dir, convert)?;
    }

    let manifest = mods::Manifest {
        id,
        name: name.clone(),
        author,
        assets: mods::ManifestAssets {
            auto_override: true,
            ..Default::default()
        },
        ..mods::default_manifest()
    };
    mods::write_manifest(&mod_dir.join("manifest.json"), &manifest)?;

    Ok(name)
}

// native as in already a jeode mod not DLL mods
fn install_native(root: &Path, mods_dir: &Path) -> AppResult<String> {
    let raw = std::fs::read_to_string(root.join("manifest.json"))?;
    let manifest: mods::Manifest = serde_json::from_str(&raw).context("invalid manifest.json")?;

    let id = mods::sanitize_id(&manifest.id);
    let dest = mods_dir.join(&id);

    if dest.exists() {
        std::fs::remove_dir_all(&dest)?;
    }
    copy_dir_recursive(root, &dest)?;

    Ok(manifest.name)
}

pub fn install(
    archive_path: &Path,
    game_dir: &Path,
    metadata: Option<&ModMetadata>,
    convert: bool,
) -> AppResult<InstallResult> {
    let staging = StagingDir::new()?;
    Archive::from_path(archive_path)?.extract(archive_path, staging.path())?;

    let mods_dir = game_dir.join("mods");
    std::fs::create_dir_all(&mods_dir)?;

    let roots = find_mod_roots(staging.path())?;
    let total = roots.len();
    let mut installed = Vec::new();
    let mut errors = Vec::new();

    for root in &roots {
        let result = if root.join("manifest.json").exists() {
            install_native(root, &mods_dir)
        } else {
            install_rebuilt(archive_path, root, game_dir, &mods_dir, metadata, convert)
        };
        match result {
            Ok(name) => installed.push(name),
            Err(e) => errors.push(e.to_string()),
        }
    }

    let error = (!errors.is_empty()).then(|| errors.join("; "));
    Ok(InstallResult {
        installed,
        total,
        error,
    })
}
