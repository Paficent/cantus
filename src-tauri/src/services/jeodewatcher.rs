use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use tauri::Emitter;

use crate::errors::AppResult;

static WATCHING: AtomicBool = AtomicBool::new(false);

const DEBOUNCE_MS: u64 = 750;

pub fn start(app: tauri::AppHandle, jeode_dir: PathBuf) -> AppResult<()> {
    if WATCHING.swap(true, Ordering::SeqCst) {
        return Ok(());
    }

    let (tx, rx) = std::sync::mpsc::channel::<notify::Result<Event>>();

    let mut watcher: RecommendedWatcher = Watcher::new(
        tx,
        notify::Config::default().with_poll_interval(Duration::from_secs(2)),
    )
    .map_err(|e| crate::errors::AppError::from(format!("Failed to create jeode watcher: {e}")))?;

    std::fs::create_dir_all(&jeode_dir).ok();

    watcher
        .watch(&jeode_dir, RecursiveMode::NonRecursive)
        .map_err(|e| crate::errors::AppError::from(format!("Failed to watch jeode dir: {e}")))?;

    std::thread::spawn(move || {
        let _watcher = watcher;
        let log_name = OsStr::new("latest.log");
        let settings_name = OsStr::new("config.json");

        let epoch = Instant::now()
            .checked_sub(Duration::from_secs(1))
            .unwrap_or_else(Instant::now);

        let mut last_log_emit = epoch;
        let mut last_settings_emit = epoch;

        while let Ok(result) = rx.recv() {
            let event = match result {
                Ok(e) => e,
                Err(_) => continue,
            };

            let now = Instant::now();

            let touches_log = event.paths.iter().any(|p| p.file_name() == Some(log_name));
            let touches_settings = event
                .paths
                .iter()
                .any(|p| p.file_name() == Some(settings_name));

            if touches_log
                && now.duration_since(last_log_emit) >= Duration::from_millis(DEBOUNCE_MS)
            {
                last_log_emit = now;
                let _ = app.emit("log-changed", ());
            }

            if touches_settings
                && now.duration_since(last_settings_emit) >= Duration::from_millis(DEBOUNCE_MS)
            {
                last_settings_emit = now;
                let _ = app.emit("jeode-settings-changed", ());
            }
        }

        WATCHING.store(false, Ordering::SeqCst);
    });

    Ok(())
}
