use tauri::State;

use crate::errors::{AppError, Context};
use crate::state::AppState;

#[tauri::command]
pub async fn read_log_file(state: State<'_, AppState>) -> Result<String, AppError> {
    let dir = state
        .game_directory
        .lock()
        .unwrap()
        .clone()
        .ok_or_else(|| AppError::from("Game directory not set"))?;

    let log_path = dir.join("jeode").join("latest.log");

    match tokio::fs::read(&log_path).await {
        Ok(bytes) => Ok(String::from_utf8_lossy(&bytes).into_owned()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
        Err(e) => Err(e).with_context(|| format!("failed to read {}", log_path.display())),
    }
}

#[tauri::command]
pub async fn watch_log_file(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let dir = state
        .game_directory
        .lock()
        .unwrap()
        .clone()
        .ok_or_else(|| AppError::from("Game directory not set"))?;

    let jeode_dir = dir.join("jeode");
    crate::services::jeodewatcher::start(app, jeode_dir)
}
