use std::path::PathBuf;

use tauri::State;
use tauri_plugin_dialog::DialogExt;

use crate::errors::AppError;
use crate::services::{game, jeode};
use crate::state::AppState;

#[tauri::command]
pub async fn select_game_directory(app: tauri::AppHandle) -> Result<Option<String>, AppError> {
    if let Some(detected) = game::detect() {
        return Ok(Some(detected.to_string_lossy().into_owned()));
    }

    let folder = app
        .dialog()
        .file()
        .set_title("Select My Singing Monsters directory")
        .blocking_pick_folder();

    match folder {
        Some(file_path) => {
            let path = file_path
                .into_path()
                .map_err(|_| AppError::from("Invalid path selected"))?;
            Ok(Some(path.to_string_lossy().into_owned()))
        }
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn validate_game_directory(
    path: String,
    state: State<'_, AppState>,
) -> Result<bool, AppError> {
    let path = PathBuf::from(&path);
    let valid = game::validate(&path);

    if valid {
        let mut dir = state.game_directory.lock().unwrap();
        *dir = Some(path);
    }

    Ok(valid)
}

#[tauri::command]
pub async fn check_jeode_installed(game_dir: String) -> Result<bool, AppError> {
    Ok(jeode::is_installed(&PathBuf::from(game_dir)))
}

#[tauri::command]
pub async fn install_jeode(game_dir: String) -> Result<(), AppError> {
    jeode::install(&PathBuf::from(game_dir)).await
}
