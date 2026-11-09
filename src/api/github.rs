use dioxus::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
pub struct GithubOwnerResponse {
    pub login: String,
    pub avatar_url: String,
    pub html_url: String,
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
pub struct GithubResponse {
    pub name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub stargazers_count: u32,
    pub forks: u32,
    pub language: String,
    pub owner: GithubOwnerResponse,
}

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[get("/api/repo?user&repo")]
pub async fn get_repo(user: String, repo: String) -> Result<GithubResponse> {
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;

    let response = client
        .get(format!("https://api.github.com/repos/{user}/{repo}"))
        .send()
        .await
        .unwrap()
        .json::<GithubResponse>()
        .await
        .unwrap();

    Ok(response)
}
