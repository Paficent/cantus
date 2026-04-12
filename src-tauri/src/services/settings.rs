use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::Manager;

use crate::errors::{AppError, AppResult};

const SETTINGS_FILE: &str = "settings.json";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub game_directory: Option<String>,

    #[serde(default)]
    pub onboarding_complete: bool,
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
