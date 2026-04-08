use std::path::{Path, PathBuf};

const MSM: &str = "MySingingMonsters.exe";

const STEAM_CANDIDATES: &[&str] = &[
    "C:\\Program Files (x86)\\Steam\\steamapps\\common\\My Singing Monsters",
    "C:\\Program Files\\Steam\\steamapps\\common\\My Singing Monsters",
    "D:\\SteamLibrary\\steamapps\\common\\My Singing Monsters",
];

pub fn validate(path: &Path) -> bool {
    path.is_dir() & path.join(MSM).exists()
}

// TODO: a real autodetection based on the NSIS registry script
pub fn detect() -> Option<PathBuf> {
    STEAM_CANDIDATES
        .iter()
        .map(PathBuf::from)
        .find(|p| validate(p))
}
