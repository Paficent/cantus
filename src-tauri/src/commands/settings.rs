use std::path::PathBuf;

use tauri::State;

use crate::errors::AppError;
use crate::services::settings::{self, JeodeSettings, Settings};
use crate::state::AppState;

fn game_dir(state: &State<'_, AppState>) -> Result<PathBuf, AppError> {
    state
        .game_directory
        .lock()
        .unwrap()
        .clone()
        .ok_or_else(|| AppError::from("Game directory not set"))
}

#[tauri::command]
pub async fn load_settings(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<Settings, AppError> {
    let settings = settings::load(&app)?;

    if let Some(ref dir) = settings.game_directory {
        let mut game_dir = state.game_directory.lock().unwrap();
        *game_dir = Some(PathBuf::from(dir));
    }

    Ok(settings)
}

#[tauri::command]
pub async fn save_settings(
    app: tauri::AppHandle,
    settings: Settings,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    if let Some(ref dir) = settings.game_directory {
        let mut game_dir = state.game_directory.lock().unwrap();
        *game_dir = Some(PathBuf::from(dir));
    }

    settings::save(&app, &settings)
}

#[tauri::command]
pub async fn read_jeode_settings(
    state: State<'_, AppState>,
) -> Result<JeodeSettings, AppError> {
    let dir = game_dir(&state)?;
    settings::load_jeode(&dir)
}

#[tauri::command]
pub async fn write_jeode_settings(
    jeode_settings: JeodeSettings,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let dir = game_dir(&state)?;
    settings::save_jeode(&dir, &jeode_settings)
}
