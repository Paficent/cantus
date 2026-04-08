use std::path::Path;

use crate::errors::{AppError, AppResult};

const GITHUB_LATEST: &str = "https://api.github.com/repos/Paficent/jeode/releases/latest";
const USER_AGENT: &str = "cantus/0.1.0";

#[derive(serde::Deserialize)]
struct Release {
    assets: Vec<Asset>,
}

#[derive(serde::Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

struct InstallTarget {
    asset_name: &'static str,
    relative_path: &'static str,
}

const TARGETS: &[InstallTarget] = &[
    InstallTarget {
        asset_name: "winhttp.dll",
        relative_path: "winhttp.dll",
    },
    InstallTarget {
        asset_name: "libjeode.dll",
        relative_path: "jeode/libjeode.dll",
    },
];

pub fn is_installed(game_dir: &Path) -> bool {
    game_dir.join("winhttp.dll").exists() || game_dir.join("jeode").join("libjeode.dll").exists()
}

pub async fn install(game_dir: &Path) -> AppResult<()> {
    let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;

    let release: Release = client
        .get(GITHUB_LATEST)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await?
        .error_for_status()
        .map_err(|e| AppError::from(format!("GitHub API request failed: {e}")))?
        .json()
        .await?;

    for target in TARGETS {
        let asset = release
            .assets
            .iter()
            .find(|a| a.name == target.asset_name)
            .ok_or_else(|| AppError::from(format!("{} not found in release", target.asset_name)))?;

        let bytes = client
            .get(&asset.browser_download_url)
            .send()
            .await?
            .error_for_status()
            .map_err(|e| AppError::from(format!("Download failed for {}: {e}", target.asset_name)))?
            .bytes()
            .await?;

        let dest = game_dir.join(target.relative_path);

        if let Some(parent) = dest.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        tokio::fs::write(&dest, &bytes).await?;
    }

    Ok(())
}
