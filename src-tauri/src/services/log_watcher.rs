use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

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
    .map_err(|e| crate::errors::AppError::from(format!("Failed to create log watcher: {e}")))?;

    std::fs::create_dir_all(&jeode_dir).ok();

    watcher
        .watch(&jeode_dir, RecursiveMode::NonRecursive)
        .map_err(|e| crate::errors::AppError::from(format!("Failed to watch log file: {e}")))?;

    std::thread::spawn(move || {
        let _watcher = watcher;
        let log_name = std::ffi::OsStr::new("latest.log");
        let mut last_emit = std::time::Instant::now()
            .checked_sub(Duration::from_secs(1))
            .unwrap_or_else(std::time::Instant::now);

        while let Ok(result) = rx.recv() {
            let event = match result {
                Ok(e) => e,
                Err(_) => continue,
            };

            let touches_log = event
                .paths
                .iter()
                .any(|p| p.file_name() == Some(log_name));

            if !touches_log {
                continue;
            }

            let now = std::time::Instant::now();
            if now.duration_since(last_emit) < Duration::from_millis(DEBOUNCE_MS) {
                continue;
            }

            last_emit = now;
            let _ = app.emit("log-changed", ());
        }

        WATCHING.store(false, Ordering::SeqCst);
    });

    Ok(())
}
