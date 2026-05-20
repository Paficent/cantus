use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use tauri::Emitter;

use crate::errors::{AppResult, Context};

static WATCHING: AtomicBool = AtomicBool::new(false);

const DEBOUNCE_MS: u64 = 500;

pub fn start(app: tauri::AppHandle, mods_dir: PathBuf) -> AppResult<()> {
    if WATCHING.swap(true, Ordering::SeqCst) {
        return Ok(());
    }

    let (tx, rx) = std::sync::mpsc::channel::<notify::Result<Event>>();

    let mut watcher: RecommendedWatcher = Watcher::new(
        tx,
        notify::Config::default().with_poll_interval(Duration::from_secs(2)),
    )
    .context("failed to create watcher")?;

    watcher
        .watch(&mods_dir, RecursiveMode::Recursive)
        .context("failed to watch mods folder")?;

    std::thread::spawn(move || {
        let _watcher = watcher;
        let mut last_emit = std::time::Instant::now()
            .checked_sub(Duration::from_secs(1))
            .unwrap_or_else(std::time::Instant::now);

        while let Ok(result) = rx.recv() {
            if result.is_err() {
                continue;
            }

            let now = std::time::Instant::now();
            if now.duration_since(last_emit) < Duration::from_millis(DEBOUNCE_MS) {
                continue;
            }

            last_emit = now;
            let _ = app.emit("mods-changed", ());
        }

        WATCHING.store(false, Ordering::SeqCst);
    });

    Ok(())
}
