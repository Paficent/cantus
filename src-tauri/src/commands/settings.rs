use crate::errors::AppError;
use crate::services::settings::{self, Settings};
use crate::state::AppState;
use std::path::PathBuf;
use tauri::State;

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
