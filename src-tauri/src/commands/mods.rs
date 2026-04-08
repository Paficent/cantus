use tauri::State;

use crate::errors::AppError;
use crate::services::mods::{self, ModInfo};
use crate::state::AppState;

fn game_dir(state: &State<'_, AppState>) -> Result<std::path::PathBuf, AppError> {
    state
        .game_directory
        .lock()
        .unwrap()
        .clone()
        .ok_or_else(|| AppError::from("Game directory not set"))
}

#[tauri::command]
pub async fn list_mods(state: State<'_, AppState>) -> Result<Vec<ModInfo>, AppError> {
    let dir = game_dir(&state)?;
    mods::list(&dir)
}

#[tauri::command]
pub async fn toggle_mod(id: String, state: State<'_, AppState>) -> Result<bool, AppError> {
    let dir = game_dir(&state)?;
    mods::toggle(&dir, &id)
}

#[tauri::command]
pub async fn remove_mod(id: String, state: State<'_, AppState>) -> Result<(), AppError> {
    let dir = game_dir(&state)?;
    mods::remove(&dir, &id)
}

#[tauri::command]
pub async fn open_mod_folder(id: String, state: State<'_, AppState>) -> Result<(), AppError> {
    let dir = game_dir(&state)?;
    let mod_path = mods::mod_folder_path(&dir, &id)?;
    tauri_plugin_opener::reveal_item_in_dir(&mod_path)
        .map_err(|e| AppError::from(format!("Failed to open mod folder: {e}")))
}

#[tauri::command]
pub async fn watch_mods_folder(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let dir = game_dir(&state)?;
    let mods_dir = dir.join("mods");
    crate::services::watcher::start(app, mods_dir)
}
