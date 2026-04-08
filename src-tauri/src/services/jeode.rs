use std::path::Path;

use sha2::{Digest, Sha256};

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

#[cfg(test)]
mod tests {
    use super::*;

    const JEODE_DOWNLOAD_BASE: &str = "https://github.com/Paficent/jeode/releases/latest/download";

    fn winhttp_asset(digest: Option<&str>) -> Asset {
        Asset {
            name: "winhttp.dll".into(),
            browser_download_url: format!("{JEODE_DOWNLOAD_BASE}/winhttp.dll"),
            digest: digest.map(String::from),
        }
    }

    fn libjeode_asset(digest: Option<&str>) -> Asset {
        Asset {
            name: "libjeode.dll".into(),
            browser_download_url: format!("{JEODE_DOWNLOAD_BASE}/libjeode.dll"),
            digest: digest.map(String::from),
        }
    }

    #[test]
    fn sha256_hex_computes_correctly() {
        let hash = sha256_hex(b"hello world");
        assert_eq!(
            hash,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn hex_encode_works() {
        assert_eq!(hex_encode(&[0x00, 0xff, 0xab]), "00ffab");
    }

    #[test]
    fn asset_parses_sha256_digest() {
        let asset = winhttp_asset(Some(
            "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        ));
        assert_eq!(
            asset.expected_sha256(),
            Some("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
        );
    }

    #[test]
    fn asset_handles_missing_digest() {
        let asset = libjeode_asset(None);
        assert_eq!(asset.expected_sha256(), None);
    }

    #[test]
    fn asset_handles_non_sha256_digest() {
        let asset = winhttp_asset(Some("md5:d41d8cd98f00b204e9800998ecf8427e"));
        assert_eq!(asset.expected_sha256(), None);
    }

    #[test]
    fn checksum_mismatch_detected() {
        let data = b"winhttp proxy dll bytes";
        let actual = sha256_hex(data);
        let bogus = "0000000000000000000000000000000000000000000000000000000000000000";
        assert_ne!(actual, bogus);
    }

    #[test]
    fn checksum_match_passes() {
        let data = b"libjeode dll bytes";
        let hash = sha256_hex(data);
        let asset = libjeode_asset(Some(&format!("sha256:{hash}")));
        assert_eq!(asset.expected_sha256(), Some(hash.as_str()));
    }
}
