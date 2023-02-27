use std::io::Read;
use reqwest::get;
use serde_json::Value;
use serde_json::json;
use crate::GitData;

pub async fn open_pull_request(github_data: &GitData, token: &String, title: &String) -> Result<String, &'static str> {
    let response = call_gh_api(github_data, token, title).await?;
    let url = get_url_from_response(response).expect("Parsing URL from response failed");

    Ok(url)
}

async fn call_gh_api(github_data: &GitData, token: &String, title: &String) -> Result<String, &'static str> {
    let body = json!({
        "title": title,
        "body": "New pull request",
        "head": github_data.branch,
        "base": github_data.main_branch
    });

    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/pulls", github_data.owner, github_data.repo_name);

    let resp = client.post(url)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", format!("{}", github_data.repo_name))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .body(body.to_string())
        .send()
        .await
        .expect("Call to GH API Failed");

    let response_text = resp.text().await.expect("Getting response.text() failed");
    Ok(response_text)
}

fn get_url_from_response(response: String) -> Result<String, &'static str> {
    let root: Value = serde_json::from_str(response.as_str()).expect("Unable to parse JSON");

    let url: Option<&str> = root.get("html_url")
        .and_then(|value| value.as_str());

    match url {
        Some(value) => Ok(String::from(value)),
        None => Err("Was not able to get URL from result")
    }
}