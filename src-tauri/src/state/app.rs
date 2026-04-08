use std::path::PathBuf;
use std::sync::Mutex;

pub struct AppState {
    pub game_directory: Mutex<Option<PathBuf>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            game_directory: Mutex::new(None),
        }
    }
}
