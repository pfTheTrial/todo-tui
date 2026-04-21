pub struct UpdateInfo {
    #[allow(dead_code)]
    pub current: String,
    pub latest: String,
    #[allow(dead_code)]
    pub url: String,
    pub has_update: bool,
}

pub fn check_for_update() -> Option<UpdateInfo> {
    let current_version = env!("CARGO_PKG_VERSION").to_string();
    let repo = "pfTheTrial/todo-tui";
    let api_url = format!("https://api.github.com/repos/{}/releases/latest", repo);

    let agent = ureq::Agent::new_with_defaults();
    let mut response = agent.get(&api_url)
        .header("User-Agent", "tdt-update-checker")
        .header("Accept", "application/vnd.github.v3+json")
        .call()
        .ok()?;

    let body: serde_json::Value = response.body_mut().read_json().ok()?;
    
    let latest_tag = body["tag_name"].as_str().unwrap_or("").trim_start_matches('v').to_string();
    let html_url = body["html_url"].as_str().unwrap_or("").to_string();

    let platform_asset = crate::utils::auto_update::get_platform_asset_name();
    let has_valid_asset = body["assets"].as_array()
        .map(|assets| assets.iter().any(|a| a["name"].as_str().unwrap_or("") == platform_asset))
        .unwrap_or(false);

    let has_update = !latest_tag.is_empty() && latest_tag != current_version && has_valid_asset;

    Some(UpdateInfo {
        current: current_version,
        latest: latest_tag,
        url: html_url,
        has_update,
    })
}
