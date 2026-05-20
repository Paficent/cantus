// https://github.com/tauri-apps/tauri/issues/10617
// https://github.com/AppImage/AppImageKit/issues/12

use crate::errors::AppError;
use crate::services::opener;

#[tauri::command]
pub async fn open_external_url(url: String) -> Result<(), AppError> {
    opener::open_url(&url)
}
