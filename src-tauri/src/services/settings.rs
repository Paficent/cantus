use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tauri::Manager;

use crate::errors::{AppError, AppResult};

const SETTINGS_FILE: &str = "config.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub game_directory: Option<String>,

    #[serde(default)]
    pub onboarding_complete: bool,

    #[serde(default)]
    pub show_nsfw: bool,

    #[serde(default = "default_theme")]
    pub theme: String,
}

fn default_theme() -> String {
    "dark".into()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            game_directory: None,
            onboarding_complete: false,
            show_nsfw: false,
            theme: default_theme(),
        }
    }
}

fn settings_path(app: &tauri::AppHandle) -> AppResult<PathBuf> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|e| AppError::from(format!("Failed to resolve config directory: {e}")))?;
    Ok(dir.join(SETTINGS_FILE))
}

pub fn load(app: &tauri::AppHandle) -> AppResult<Settings> {
    let path = settings_path(app)?;

    if !path.exists() {
        return Ok(Settings::default());
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| AppError::from(format!("Failed to read settings: {e}")))?;

    serde_json::from_str(&content)
        .map_err(|e| AppError::from(format!("Failed to parse settings: {e}")))
}

pub fn save(app: &tauri::AppHandle, settings: &Settings) -> AppResult<()> {
    let path = settings_path(app)?;

    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }

    let content = serde_json::to_string_pretty(settings)?;
    std::fs::write(&path, content + "\n")?;

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JeodeSettings {
    #[serde(default)]
    pub last_update_check: u64,

    #[serde(default = "default_true")]
    pub overlays_enabled: bool,

    #[serde(default)]
    pub debug: bool,

    #[serde(default)]
    pub allow_unsafe_functions: bool,

    #[serde(default)]
    pub suppress_native_warnings: bool,

    #[serde(default = "default_toggle_key")]
    pub toggle_key: String,
}

fn default_true() -> bool {
    true
}

fn default_toggle_key() -> String {
    "Tilde".into()
}

impl Default for JeodeSettings {
    fn default() -> Self {
        Self {
            last_update_check: 0,
            overlays_enabled: true,
            debug: false,
            allow_unsafe_functions: false,
            suppress_native_warnings: false,
            toggle_key: default_toggle_key(),
        }
    }
}

fn jeode_settings_path(game_dir: &Path) -> PathBuf {
    game_dir.join("jeode").join("config.json")
}

pub fn load_jeode(game_dir: &Path) -> AppResult<JeodeSettings> {
    let path = jeode_settings_path(game_dir);

    if !path.exists() {
        return Ok(JeodeSettings::default());
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| AppError::from(format!("Failed to read jeode settings: {e}")))?;

    serde_json::from_str(&content)
        .map_err(|e| AppError::from(format!("Failed to parse jeode settings: {e}")))
}

pub fn save_jeode(game_dir: &Path, settings: &JeodeSettings) -> AppResult<()> {
    let path = jeode_settings_path(game_dir);

    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }

    let content = serde_json::to_string_pretty(settings)?;
    std::fs::write(&path, content + "\n")?;

    Ok(())
}
