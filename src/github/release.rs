use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Release {
    pub tag_name: String,
}

pub fn fetch_releases(repo: &str) -> Result<Vec<Release>, Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/repos/{}/releases", repo);

    let client = Client::new();

    let res = client
        .get(&url)
        .header("User-Agent", "weakpm") // REQUIRED by GitHub
        .send()?
        .json::<Vec<Release>>()?;

    Ok(res)
}
