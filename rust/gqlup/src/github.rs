const REPO_API_URL: &str =
    "https://api.github.com/repos/Voldemat/graphql-plus-plus/releases/latest";
const USER_AGENT: &str = "gqlup";

#[derive(serde::Deserialize)]
pub struct Asset {
    pub name: String,
    pub browser_download_url: String,
}

#[derive(serde::Deserialize)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<Asset>,
}

pub fn fetch_latest_release(
    client: &reqwest::blocking::Client,
) -> Result<Release, String> {
    let response = client
        .get(REPO_API_URL)
        .header("User-Agent", USER_AGENT)
        .header("Accept", "application/vnd.github+json")
        .send()
        .map_err(|e| format!("Failed to reach GitHub: {e}"))?;
    if !response.status().is_success() {
        return Err(format!(
            "GitHub API returned status {}",
            response.status()
        ));
    }
    let body = response
        .text()
        .map_err(|e| format!("Failed to read GitHub response: {e}"))?;
    serde_json::from_str(&body)
        .map_err(|e| format!("Failed to parse GitHub response: {e}"))
}

pub fn download_asset(
    client: &reqwest::blocking::Client,
    asset: &Asset,
) -> Result<Vec<u8>, String> {
    let response = client
        .get(asset.browser_download_url.as_str())
        .header("User-Agent", USER_AGENT)
        .send()
        .map_err(|e| format!("Failed to download {}: {e}", asset.name))?;
    if !response.status().is_success() {
        return Err(format!(
            "Failed to download {}: status {}",
            asset.name,
            response.status()
        ));
    }
    response
        .bytes()
        .map(|b| b.to_vec())
        .map_err(|e| format!("Failed to read downloaded asset: {e}"))
}
