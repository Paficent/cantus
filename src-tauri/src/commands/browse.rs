use tauri::State;

use crate::errors::AppError;
use crate::services::gamebanana::{self, BrowsePage, CategoryInfo};
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
pub async fn browse_mods(
    page: u32,
    per_page: u32,
    sort: String,
    search: String,
    category_id: Option<u64>,
    show_nsfw: bool,
) -> Result<BrowsePage, AppError> {
    let trimmed = search.trim();
    if !trimmed.is_empty() {
        return gamebanana::search_mods(trimmed, page, per_page, show_nsfw).await;
    }

    gamebanana::list_mods(page, per_page, &sort, category_id, show_nsfw).await
}

#[tauri::command]
pub async fn browse_categories() -> Result<Vec<CategoryInfo>, AppError> {
    gamebanana::get_categories().await
}

use crate::services::installer::InstallResult;

#[tauri::command]
pub async fn browse_install_mod(
    mod_id: u64,
    mod_name: String,
    mod_author: String,
    state: State<'_, AppState>,
) -> Result<InstallResult, AppError> {
    let dir = game_dir(&state)?;

    let (file_id, file_name) = gamebanana::pick_mod_file(mod_id).await?;
    let archive_path = gamebanana::download_to_temp(file_id, &file_name).await?;

    let metadata = crate::services::installer::ModMetadata {
        id: format!("gb-{mod_id}"),
        name: mod_name,
        author: mod_author,
    };

    let result = crate::services::installer::install(&archive_path, &dir, Some(&metadata));
    let _ = std::fs::remove_file(&archive_path);
    result
}
