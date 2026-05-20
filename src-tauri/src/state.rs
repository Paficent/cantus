use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Default)]
pub struct AppState {
    pub game_directory: Mutex<Option<PathBuf>>,
}
