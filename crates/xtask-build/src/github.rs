use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub fn latest_release_url(user: &str, repo: &str) -> String {
    format!("https://api.github.com/repos/{user}/{repo}/releases/latest")
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub author_type: String,
    pub site_admin: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Asset {
    pub url: String,
    pub id: u64,
    pub node_id: String,
    pub name: String,
    pub label: String,
    pub uploader: User,
    pub content_type: String,
    pub state: String,
    pub size: u64,
    pub download_count: u64,
    pub created_at: String,
    pub updated_at: String,
    pub browser_download_url: String,
}

#[derive(Deserialize, Serialize)]
pub struct Release {
    pub url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub html_url: String,
    pub id: u64,
    pub author: User,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub assets: Vec<Asset>,
    pub tarball_url: String,
    pub zipball_url: String,
    pub body: String,
}
