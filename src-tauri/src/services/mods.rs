use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::errors::{AppError, AppResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(default = "default_unknown")]
    pub id: String,
    #[serde(default = "default_unknown")]
    pub name: String,
    #[serde(default = "default_unknown")]
    pub author: String,
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub game_version: String,
    #[serde(default = "default_entry")]
    pub entry: String,
    #[serde(default)]
    pub native_entry: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_true")]
    pub error_on_game_update: bool,
    #[serde(default)]
    pub load_priority: i32,
    #[serde(default = "default_schema")]
    pub schema_version: i32,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub assets: ManifestAssets,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ManifestAssets {
    #[serde(default = "default_true")]
    pub auto_override: bool,
    #[serde(default)]
    pub overrides: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub dat_overrides: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ModInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    #[serde(rename = "type")]
    pub mod_type: String,
    pub enabled: bool,
}

fn default_unknown() -> String {
    "Unknown".into()
}
fn default_version() -> String {
    "1.0.0".into()
}
fn default_entry() -> String {
    "init.lua".into()
}
fn default_true() -> bool {
    true
}
fn default_schema() -> i32 {
    1
}

fn detect_type(manifest: &Manifest, mod_path: &Path) -> String {
    if !manifest.native_entry.is_empty() {
        return "native".into();
    }
    if mod_path.join(&manifest.entry).exists() {
        return "lua".into();
    }
    "asset".into()
}

fn sanitize_id(raw: &str) -> String {
    let out: String = raw
        .chars()
        .filter_map(|c| {
            let lc = c.to_ascii_lowercase();
            if lc.is_ascii_alphanumeric() || lc == '_' || lc == '-' {
                Some(lc)
            } else {
                None
            }
        })
        .collect();
    if out.is_empty() {
        "unknown".into()
    } else {
        out
    }
}

fn read_manifest(mod_path: &Path, dir_name: &str) -> AppResult<Manifest> {
    let manifest_path = mod_path.join("manifest.json");
    if !manifest_path.exists() {
        return Ok(Manifest {
            id: sanitize_id(dir_name),
            name: dir_name.into(),
            ..default_manifest()
        });
    }
    let content = std::fs::read_to_string(&manifest_path)?;
    let mut manifest: Manifest = serde_json::from_str(&content)
        .map_err(|e| AppError::from(format!("Failed to parse manifest in {dir_name}: {e}")))?;
    manifest.id = sanitize_id(&manifest.id);
    Ok(manifest)
}

fn default_manifest() -> Manifest {
    Manifest {
        id: "unknown".into(),
        name: "Unknown".into(),
        author: "Unknown".into(),
        version: "1.0.0".into(),
        game_version: String::new(),
        entry: "init.lua".into(),
        native_entry: String::new(),
        enabled: true,
        error_on_game_update: true,
        load_priority: 0,
        schema_version: 1,
        dependencies: Vec::new(),
        assets: ManifestAssets::default(),
    }
}

pub fn list(game_dir: &Path) -> AppResult<Vec<ModInfo>> {
    let mods_dir = game_dir.join("mods");
    if !mods_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut mods = Vec::new();
    let entries = std::fs::read_dir(&mods_dir)?;

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let dir_name = entry.file_name().to_string_lossy().to_string();
        let manifest = match read_manifest(&path, &dir_name) {
            Ok(m) => m,
            Err(e) => {
                log::warn!("Skipping mod {dir_name}: {e}");
                continue;
            }
        };

        let mod_type = detect_type(&manifest, &path);

        mods.push(ModInfo {
            id: manifest.id,
            name: manifest.name,
            version: manifest.version,
            author: manifest.author,
            mod_type,
            enabled: manifest.enabled,
        });
    }

    mods.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(mods)
}

pub fn toggle(game_dir: &Path, mod_id: &str) -> AppResult<bool> {
    let mod_path = find_mod_dir(game_dir, mod_id)?;
    let manifest_path = mod_path.join("manifest.json");

    let mut manifest = read_manifest(&mod_path, mod_id)?;
    manifest.enabled = !manifest.enabled;
    write_manifest(&manifest_path, &manifest)?;

    Ok(manifest.enabled)
}

pub fn remove(game_dir: &Path, mod_id: &str) -> AppResult<()> {
    let mod_path = find_mod_dir(game_dir, mod_id)?;
    std::fs::remove_dir_all(&mod_path)?;
    Ok(())
}

pub fn mod_folder_path(game_dir: &Path, mod_id: &str) -> AppResult<PathBuf> {
    find_mod_dir(game_dir, mod_id)
}

fn find_mod_dir(game_dir: &Path, mod_id: &str) -> AppResult<PathBuf> {
    let mods_dir = game_dir.join("mods");
    let entries = std::fs::read_dir(&mods_dir)?;

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let dir_name = entry.file_name().to_string_lossy().to_string();
        let manifest_path = path.join("manifest.json");
        if manifest_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&manifest_path) {
                if let Ok(m) = serde_json::from_str::<Manifest>(&content) {
                    if sanitize_id(&m.id) == mod_id {
                        return Ok(path);
                    }
                }
            }
        }
        if sanitize_id(&dir_name) == mod_id {
            return Ok(path);
        }
    }

    Err(AppError::from(format!("Mod '{mod_id}' not found")))
}

fn write_manifest(path: &Path, manifest: &Manifest) -> AppResult<()> {
    let content = serde_json::to_string_pretty(manifest)?;
    std::fs::write(path, content + "\n")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup_mod(
        tmp: &Path,
        dir_name: &str,
        manifest_json: Option<&str>,
        create_entry: bool,
    ) -> PathBuf {
        let mods_dir = tmp.join("mods");
        let mod_dir = mods_dir.join(dir_name);
        fs::create_dir_all(&mod_dir).unwrap();
        if let Some(json) = manifest_json {
            fs::write(mod_dir.join("manifest.json"), json).unwrap();
        }
        if create_entry {
            fs::write(mod_dir.join("init.lua"), "-- entry").unwrap();
        }
        mod_dir
    }

    #[test]
    fn sanitize_id_strips_invalid_chars() {
        assert_eq!(sanitize_id("My Cool Mod!"), "mycoolmod");
        assert_eq!(sanitize_id("test-mod_123"), "test-mod_123");
        assert_eq!(sanitize_id("!!!"), "unknown");
        assert_eq!(sanitize_id(""), "unknown");
    }

    #[test]
    fn detect_type_native_when_native_entry_set() {
        let tmp = tempfile::tempdir().unwrap();
        let mod_path = tmp.path().join("test-mod");
        fs::create_dir_all(&mod_path).unwrap();
        let manifest = Manifest {
            native_entry: "mod.dll".into(),
            ..default_manifest()
        };
        assert_eq!(detect_type(&manifest, &mod_path), "native");
    }

    #[test]
    fn detect_type_lua_when_entry_exists() {
        let tmp = tempfile::tempdir().unwrap();
        let mod_path = tmp.path().join("test-mod");
        fs::create_dir_all(&mod_path).unwrap();
        fs::write(mod_path.join("init.lua"), "").unwrap();
        let manifest = Manifest {
            native_entry: String::new(),
            entry: "init.lua".into(),
            ..default_manifest()
        };
        assert_eq!(detect_type(&manifest, &mod_path), "lua");
    }

    #[test]
    fn detect_type_asset_when_no_entry() {
        let tmp = tempfile::tempdir().unwrap();
        let mod_path = tmp.path().join("test-mod");
        fs::create_dir_all(&mod_path).unwrap();
        let manifest = Manifest {
            native_entry: String::new(),
            entry: "init.lua".into(),
            ..default_manifest()
        };
        assert_eq!(detect_type(&manifest, &mod_path), "asset");
    }

    #[test]
    fn list_reads_mods_from_directory() {
        let tmp = tempfile::tempdir().unwrap();
        setup_mod(
            tmp.path(),
            "my-mod",
            Some(
                r#"{"id":"my-mod","name":"My Mod","author":"Tester","version":"1.0.0","enabled":true}"#,
            ),
            true,
        );
        setup_mod(
            tmp.path(),
            "asset-pack",
            Some(
                r#"{"id":"asset-pack","name":"Asset Pack","author":"Artist","version":"2.0.0","enabled":false}"#,
            ),
            false,
        );

        let mods = list(tmp.path()).unwrap();
        assert_eq!(mods.len(), 2);

        let asset = mods.iter().find(|m| m.id == "asset-pack").unwrap();
        assert_eq!(asset.mod_type, "asset");
        assert!(!asset.enabled);

        let cool = mods.iter().find(|m| m.id == "my-mod").unwrap();
        assert_eq!(cool.mod_type, "lua");
        assert!(cool.enabled);
    }

    #[test]
    fn list_returns_empty_when_no_mods_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let mods = list(tmp.path()).unwrap();
        assert!(mods.is_empty());
    }

    #[test]
    fn list_generates_manifest_for_dir_without_one() {
        let tmp = tempfile::tempdir().unwrap();
        setup_mod(tmp.path(), "bare-mod", None, false);

        let mods = list(tmp.path()).unwrap();
        assert_eq!(mods.len(), 1);
        assert_eq!(mods[0].id, "bare-mod");
        assert_eq!(mods[0].name, "bare-mod");
        assert_eq!(mods[0].mod_type, "asset");
    }

    #[test]
    fn toggle_flips_enabled_in_manifest() {
        let tmp = tempfile::tempdir().unwrap();
        setup_mod(
            tmp.path(),
            "toggle-test",
            Some(r#"{"id":"toggle-test","name":"Toggle Test","enabled":true}"#),
            false,
        );

        let new_state = toggle(tmp.path(), "toggle-test").unwrap();
        assert!(!new_state);

        let mods = list(tmp.path()).unwrap();
        assert!(!mods[0].enabled);

        let new_state = toggle(tmp.path(), "toggle-test").unwrap();
        assert!(new_state);
    }

    #[test]
    fn remove_deletes_mod_directory() {
        let tmp = tempfile::tempdir().unwrap();
        setup_mod(
            tmp.path(),
            "remove-me",
            Some(r#"{"id":"remove-me","name":"test"}"#),
            true,
        );

        assert!(tmp.path().join("mods/remove-me").exists());
        remove(tmp.path(), "remove-me").unwrap();
        assert!(!tmp.path().join("mods/remove-me").exists());
    }
}
