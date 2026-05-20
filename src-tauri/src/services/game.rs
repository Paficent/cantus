use std::path::{Path, PathBuf};

const GAME_EXE: &str = "MySingingMonsters.exe";
const GAME_FOLDER: &str = "My Singing Monsters";

pub fn validate(path: &Path) -> bool {
    path.is_dir() && path.join(GAME_EXE).exists()
}

pub fn detect() -> Option<PathBuf> {
    detect_platform()
}

fn msm_in_library(library: &Path) -> Option<PathBuf> {
    let candidate = library.join("steamapps").join("common").join(GAME_FOLDER);
    validate(&candidate).then_some(candidate)
}

#[cfg(windows)]
fn detect_platform() -> Option<PathBuf> {
    steam_path_from_registry()
        .and_then(|steam| msm_in_library(&steam).or_else(|| scan_library_folders(&steam)))
}

#[cfg(not(windows))]
fn detect_platform() -> Option<PathBuf> {
    let home = std::env::var("HOME").ok()?;
    let candidates = [
        PathBuf::from(&home).join(".steam/steam"),
        PathBuf::from(&home).join(".local/share/Steam"),
    ];
    candidates
        .into_iter()
        .find_map(|steam| msm_in_library(&steam).or_else(|| scan_library_folders(&steam)))
}

#[cfg(windows)]
fn steam_path_from_registry() -> Option<PathBuf> {
    use winreg::enums::*;
    use winreg::RegKey;

    let pairs: &[(_, &str)] = &[
        (HKEY_LOCAL_MACHINE, r"SOFTWARE\WOW6432Node\Valve\Steam"),
        (HKEY_LOCAL_MACHINE, r"SOFTWARE\Valve\Steam"),
        (HKEY_CURRENT_USER, r"SOFTWARE\Valve\Steam"),
    ];

    for &(root, subkey) in pairs {
        if let Ok(key) = RegKey::predef(root).open_subkey(subkey) {
            if let Ok(path) = key.get_value::<String, _>("InstallPath") {
                let p = PathBuf::from(path);
                if p.is_dir() {
                    return Some(p);
                }
            }
        }
    }

    None
}

fn scan_library_folders(steam_root: &Path) -> Option<PathBuf> {
    let vdf = steam_root.join("steamapps").join("libraryfolders.vdf");
    let content = std::fs::read_to_string(vdf).ok()?;
    parse_library_paths(&content)
        .into_iter()
        .find_map(|lib| msm_in_library(&lib))
}

fn parse_library_paths(vdf_content: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for line in vdf_content.lines() {
        let trimmed = line.trim();
        if !trimmed.contains("\"path\"") {
            continue;
        }
        if let Some(value) = extract_vdf_value(trimmed) {
            let cleaned = value.replace("\\\\", "\\");
            paths.push(PathBuf::from(cleaned));
        }
    }
    paths
}

fn extract_vdf_value(line: &str) -> Option<&str> {
    let mut quotes = line.match_indices('"');
    let _ = quotes.next()?;
    let _ = quotes.next()?;
    let (start, _) = quotes.next()?;
    let (end, _) = quotes.next()?;
    Some(&line[start + 1..end])
}
