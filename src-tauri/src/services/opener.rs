// https://github.com/tauri-apps/tauri/issues/10617
// https://github.com/AppImage/AppImageKit/issues/12

use crate::errors::{AppResult, Context};

// Fine even when running as dev
#[cfg(target_os = "linux")]
const STRIPPED_ENV: &[&str] = &[
    "LD_LIBRARY_PATH",
    "LD_PRELOAD",
    "APPDIR",
    "ARGV0",
    "OWD",
    "PYTHONHOME",
    "PYTHONPATH",
    "PERLLIB",
    "GSETTINGS_SCHEMA_DIR",
    "GIO_EXTRA_MODULES",
    "GST_PLUGIN_SYSTEM_PATH",
    "GST_PLUGIN_SYSTEM_PATH_1_0",
    "QT_PLUGIN_PATH",
    "XDG_DATA_DIRS",
];

#[cfg(target_os = "linux")]
pub fn open_url(url: &str) -> AppResult<()> {
    use std::process::{Command, Stdio};

    let mut cmd = Command::new("xdg-open");
    cmd.arg(url);
    for var in STRIPPED_ENV {
        cmd.env_remove(var);
    }
    cmd.stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .context("failed to spawn xdg-open")?;
    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn open_url(url: &str) -> AppResult<()> {
    tauri_plugin_opener::open_url(url, None::<&str>).context("failed to open url")?;
    Ok(())
}
