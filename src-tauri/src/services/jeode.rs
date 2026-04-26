use std::path::Path;

use sha2::{Digest, Sha256};

use crate::errors::{AppError, AppResult, Context};

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
    digest: Option<String>,
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
        .context("GitHub API request failed")?
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
            .with_context(|| format!("download failed for {}", target.asset_name))?
            .bytes()
            .await?;

        if let Some(expected) = asset.expected_sha256() {
            let actual = sha256_hex(&bytes);
            if actual != expected {
                return Err(AppError::from(format!(
                    "Checksum mismatch for {}: expected {expected}, got {actual}",
                    target.asset_name
                )));
            }
        }

        let dest = game_dir.join(target.relative_path);

        if let Some(parent) = dest.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        tokio::fs::write(&dest, &bytes).await?;
    }

    tokio::fs::create_dir_all(game_dir.join("mods")).await?;

    Ok(())
}

impl Asset {
    fn expected_sha256(&self) -> Option<&str> {
        self.digest
            .as_deref()
            .and_then(|d| d.strip_prefix("sha256:"))
    }
}

fn sha256_hex(data: &[u8]) -> String {
    let hash = Sha256::digest(data);
    hex_encode(&hash)
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}
