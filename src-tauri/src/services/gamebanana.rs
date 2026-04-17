use serde::{Deserialize, Serialize};

use crate::errors::{AppError, AppResult};

const API_BASE: &str = "https://gamebanana.com/apiv11";
const MSM_GAME_ID: u32 = 9640;

#[derive(Debug, Clone, Serialize)]
pub struct BrowseMod {
    pub id: u64,
    pub name: String,
    pub author: String,
    pub views: u64,
    pub likes: u64,
    pub downloads: u64,
    pub category: String,
    pub screenshot: String,
    pub date_added: u64,
    pub date_updated: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct BrowsePage {
    pub mods: Vec<BrowseMod>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct CategoryInfo {
    pub id: u64,
    pub name: String,
}

#[derive(Deserialize)]
struct ApiListResponse {
    #[serde(default, rename = "_aMetadata")]
    metadata: Option<ApiMetadata>,
    #[serde(default, rename = "_aRecords")]
    records: Vec<serde_json::Value>,
}

#[derive(Deserialize)]
struct ApiMetadata {
    #[serde(default, rename = "_bIsComplete")]
    is_complete: Option<bool>,
}

fn str_field(val: &serde_json::Value, key: &str) -> String {
    val.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

fn u64_field(val: &serde_json::Value, key: &str) -> u64 {
    match val.get(key) {
        Some(serde_json::Value::Number(n)) => n
            .as_u64()
            .or_else(|| n.as_f64().map(|f| f as u64))
            .unwrap_or(0),
        Some(serde_json::Value::String(s)) => s.replace(',', "").trim().parse().unwrap_or(0),
        _ => 0,
    }
}

fn first_screenshot(record: &serde_json::Value) -> String {
    let media = match record.get("_aPreviewMedia") {
        Some(m) => m,
        None => return String::new(),
    };

    let images = media
        .get("_aImages")
        .and_then(|v| v.as_array())
        .or_else(|| media.as_array());

    let first = match images.and_then(|arr| arr.first()) {
        Some(img) => img,
        None => return String::new(),
    };

    let base = match first.get("_sBaseUrl").and_then(|v| v.as_str()) {
        Some(b) => b,
        None => return String::new(),
    };

    let file = first
        .get("_sFile530")
        .or_else(|| first.get("_sFile220"))
        .or_else(|| first.get("_sFile"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if file.is_empty() {
        return String::new();
    }

    format!("{base}/{file}")
}

fn parse_mod(record: &serde_json::Value) -> Option<BrowseMod> {
    let model = str_field(record, "_sModelName");
    if !model.is_empty() && model != "Mod" {
        return None;
    }

    let game = record.get("_aGame").unwrap_or(&serde_json::Value::Null);
    let game_id = u64_field(game, "_idRow");
    if game_id != 0 && game_id != MSM_GAME_ID as u64 {
        return None;
    }

    let submitter = record
        .get("_aSubmitter")
        .unwrap_or(&serde_json::Value::Null);
    let root_category = record
        .get("_aRootCategory")
        .unwrap_or(&serde_json::Value::Null);

    let date_updated = u64_field(record, "_tsDateUpdated");
    let date_modified = u64_field(record, "_tsDateModified");

    Some(BrowseMod {
        id: u64_field(record, "_idRow"),
        name: str_field(record, "_sName"),
        author: str_field(submitter, "_sName"),
        views: u64_field(record, "_nViewCount").max(u64_field(record, "_nViews")),
        likes: u64_field(record, "_nLikeCount").max(u64_field(record, "_nLikes")),
        downloads: u64_field(record, "_nDownloadCount").max(u64_field(record, "_nDownloads")),
        category: str_field(root_category, "_sName"),
        screenshot: first_screenshot(record),
        date_added: u64_field(record, "_tsDateAdded"),
        date_updated: date_updated.max(date_modified),
    })
}

fn is_nsfw_content(record: &serde_json::Value) -> bool {
    let visibility = str_field(record, "_sInitialVisibility");
    visibility == "warn" || visibility == "hide"
}

fn to_gb_sort(sort: &str) -> &str {
    match sort {
        "popular" => "Generic_MostViewed",
        "newest" => "Generic_Latest",
        "updated" => "Generic_LatestUpdated",
        "downloads" => "Generic_MostDownloaded",
        "likes" => "Generic_MostLiked",
        _ => "Generic_NewAndUpdated",
    }
}

fn has_more_from(metadata: &Option<ApiMetadata>, fetched: usize, per_page: u32) -> bool {
    metadata
        .as_ref()
        .and_then(|m| m.is_complete)
        .map(|complete| !complete)
        .unwrap_or(fetched == per_page as usize)
}

async fn hydrate_download_counts(mods: &mut [BrowseMod]) {
    let tasks: Vec<_> = mods
        .iter()
        .map(|m| {
            let id = m.id;
            tokio::spawn(async move {
                let url = format!("{API_BASE}/Mod/{id}?_csvProperties=_nDownloadCount");
                let val: serde_json::Value = reqwest::get(&url).await?.json().await?;
                Ok::<_, reqwest::Error>((id, val))
            })
        })
        .collect();

    for task in tasks {
        let Ok(Ok((id, val))) = task.await else {
            continue;
        };
        if let Some(m) = mods.iter_mut().find(|m| m.id == id) {
            m.downloads = u64_field(&val, "_nDownloadCount");
        }
    }
}

pub async fn list_mods(
    page: u32,
    per_page: u32,
    sort: &str,
    category_id: Option<u64>,
    show_nsfw: bool,
) -> AppResult<BrowsePage> {
    let clamped = per_page.clamp(1, 50);
    let gb_sort = to_gb_sort(sort);

    let mut url = format!(
        "{API_BASE}/Mod/Index?_nPage={page}&_nPerpage={clamped}&_sSort={gb_sort}&_idGameRow={MSM_GAME_ID}&_aFilters[Generic_Game]={MSM_GAME_ID}"
    );

    if let Some(cat) = category_id {
        url.push_str(&format!("&_aFilters[Generic_Category]={cat}"));
    }

    let response: ApiListResponse = reqwest::get(&url).await?.json().await?;
    let mut mods: Vec<BrowseMod> = response
        .records
        .iter()
        .filter(|r| show_nsfw || !is_nsfw_content(r))
        .filter_map(parse_mod)
        .collect();
    let has_more = has_more_from(&response.metadata, mods.len(), clamped);
    hydrate_download_counts(&mut mods).await;

    Ok(BrowsePage { mods, has_more })
}

pub async fn search_mods(
    query: &str,
    page: u32,
    per_page: u32,
    show_nsfw: bool,
) -> AppResult<BrowsePage> {
    let trimmed = query.trim();
    if trimmed.len() < 2 {
        return Ok(BrowsePage {
            mods: vec![],
            has_more: false,
        });
    }

    let clamped = per_page.clamp(1, 50);
    let encoded = urlencoding::encode(trimmed);

    let url = format!(
        "{API_BASE}/Util/Search/Results?_sSearchString={encoded}&_nPage={page}&_nPerpage={clamped}&_sModelName=Mod&_idGameRow={MSM_GAME_ID}&_sOrder=best_match"
    );

    let response: ApiListResponse = reqwest::get(&url).await?.json().await?;
    let mut mods: Vec<BrowseMod> = response
        .records
        .iter()
        .filter(|r| show_nsfw || !is_nsfw_content(r))
        .filter_map(parse_mod)
        .collect();
    let has_more = has_more_from(&response.metadata, mods.len(), clamped);
    hydrate_download_counts(&mut mods).await;

    Ok(BrowsePage { mods, has_more })
}

pub async fn get_categories() -> AppResult<Vec<CategoryInfo>> {
    let url = format!("{API_BASE}/Mod/Categories?_sSort=a_to_z&_idGameRow={MSM_GAME_ID}");
    let records: Vec<serde_json::Value> = reqwest::get(&url).await?.json().await?;

    let categories = records
        .iter()
        .filter_map(|entry| {
            let name = str_field(entry, "_sName");
            if name.is_empty() {
                return None;
            }

            let id = str_field(entry, "_sUrl")
                .rsplit('/')
                .next()
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or_else(|| u64_field(entry, "_idRow"));

            if id == 0 {
                return None;
            }

            Some(CategoryInfo { id, name })
        })
        .collect();

    Ok(categories)
}

const SUPPORTED_EXTENSIONS: &[&str] = &["zip", "7z", "rar"];

fn has_supported_extension(filename: &str) -> bool {
    let lower = filename.to_lowercase();
    SUPPORTED_EXTENSIONS
        .iter()
        .any(|ext| lower.ends_with(&format!(".{ext}")))
}

pub async fn pick_mod_file(mod_id: u64) -> AppResult<(u64, String)> {
    let url = format!("{API_BASE}/Mod/{mod_id}/Files");
    let files: Vec<serde_json::Value> = reqwest::get(&url).await?.json().await?;

    let best = files
        .iter()
        .filter(|f| has_supported_extension(&str_field(f, "_sFile")))
        .max_by_key(|f| u64_field(f, "_tsDateAdded"));

    let file = match best {
        Some(f) => f,
        None => {
            return Err(AppError::from(
                "No supported archive found (.zip, .7z, .rar)",
            ))
        }
    };

    let file_id = u64_field(file, "_idRow");
    let file_name = str_field(file, "_sFile");

    if file_id == 0 || file_name.is_empty() {
        return Err(AppError::from("Invalid file metadata from GameBanana"));
    }

    Ok((file_id, file_name))
}

pub async fn download_to_temp(file_id: u64, file_name: &str) -> AppResult<std::path::PathBuf> {
    let download_url = format!("https://gamebanana.com/dl/{file_id}");
    let response = reqwest::get(&download_url).await?;

    if !response.status().is_success() {
        return Err(AppError::from(format!(
            "Download failed (HTTP {})",
            response.status()
        )));
    }

    let temp_dir = std::env::temp_dir().join("cantus/downloads");
    std::fs::create_dir_all(&temp_dir)?;

    let dest = temp_dir.join(file_name);
    let bytes = response.bytes().await?;
    tokio::fs::write(&dest, &bytes)
        .await
        .map_err(|e| AppError::from(format!("Failed to write download: {e}")))?;

    Ok(dest)
}
