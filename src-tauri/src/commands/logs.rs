use tauri::State;

use crate::errors::AppError;
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

    if !log_path.exists() {
        return Ok(String::new());
    }

    std::fs::read_to_string(&log_path)
        .map_err(|e| AppError::from(format!("Failed to read log file: {e}")))
}
