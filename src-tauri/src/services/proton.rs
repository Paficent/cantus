use std::path::{Path, PathBuf};

use crate::errors::{AppError, AppResult, Context};

const APP_ID: &str = "1419170";
const OVERRIDE_SECTION: &str = "[Software\\\\Wine\\\\DllOverrides]";
const OVERRIDE_ENTRY: &str = "\"winhttp\"=\"native,builtin\"";

#[cfg(target_os = "linux")]
pub fn step_needed(game_dir: &Path) -> bool {
    find_prefix(game_dir).is_some()
}

#[cfg(not(target_os = "linux"))]
pub fn step_needed(_game_dir: &Path) -> bool {
    false
}

pub fn apply_winhttp_override(game_dir: &Path) -> AppResult<()> {
    let prefix = find_prefix(game_dir)
        .ok_or_else(|| AppError::from("Could not find a Wine prefix. Are you using proton?"))?;
    let reg = prefix.join("user.reg");

    let content = std::fs::read_to_string(&reg).context("failed to read user.reg")?;

    if let Some(updated) = with_override(&content) {
        std::fs::write(&reg, updated).context("failed to write user.reg")?;
    }

    Ok(())
}

fn find_prefix(game_dir: &Path) -> Option<PathBuf> {
    if let Some(steamapps) = game_dir.parent()?.parent() {
        let prefix = steamapps.join("compatdata").join(APP_ID).join("pfx");
        if prefix.join("user.reg").is_file() {
            return Some(prefix);
        }
    }

    if let Ok(explicit) = std::env::var("WINEPREFIX") {
        let prefix = PathBuf::from(explicit);
        if prefix.join("user.reg").is_file() {
            return Some(prefix);
        }
    }

    let prefix = PathBuf::from(std::env::var("HOME").ok()?).join(".wine");
    prefix.join("user.reg").is_file().then_some(prefix)
}

fn with_override(content: &str) -> Option<String> {
    let mut lines: Vec<&str> = content.lines().collect();

    match lines.iter().position(|l| l.starts_with(OVERRIDE_SECTION)) {
        Some(header) => {
            let end = lines[header + 1..]
                .iter()
                .position(|l| l.starts_with('['))
                .map(|offset| header + 1 + offset)
                .unwrap_or(lines.len());

            if lines[header..end]
                .iter()
                .any(|l| l.trim_start().starts_with("\"winhttp\""))
            {
                return None;
            }

            let at = match lines.get(header + 1) {
                Some(l) if l.starts_with("#time=") => header + 2,
                _ => header + 1,
            };
            lines.insert(at, OVERRIDE_ENTRY);
        }
        None => {
            if lines.last().is_some_and(|l| !l.is_empty()) {
                lines.push("");
            }
            lines.push(OVERRIDE_SECTION);
            lines.push(OVERRIDE_ENTRY);
        }
    }

    Some(lines.join("\n") + "\n")
}
