mod commands;
mod errors;
mod services;
mod state;

use commands::{logs, mods, onboarding, settings};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(state::AppState::default())
        .invoke_handler(tauri::generate_handler![
            onboarding::select_game_directory,
            onboarding::detect_game_directory,
            onboarding::browse_game_directory,
            onboarding::validate_game_directory,
            onboarding::check_jeode_installed,
            onboarding::install_jeode,
            onboarding::launch_game,
            mods::list_mods,
            mods::toggle_mod,
            mods::remove_mod,
            mods::open_mod_folder,
            mods::watch_mods_folder,
            mods::install_mod,
            settings::load_settings,
            settings::save_settings,
            settings::read_jeode_settings,
            settings::write_jeode_settings,
            logs::read_log_file,
            logs::watch_log_file,
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
