use reqwest::{Response, StatusCode};
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

    let status = resp.status().to_string();
    let body = resp.text().await.expect("Getting response.text() failed");

    handle_api_error(&status, &body).await?;

    Ok(body)
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

async fn handle_api_error(status: &String, body: &String) -> Result<(), &'static str> {
    if status.contains("422") {
        if body.contains("head") && body.contains("invalid") {
            return Err("Branch name does not exist on remote. Make sure to push changes first");
        } else if body.contains("A pull request already exists") {
            return Err("Pull Request already exists");
        }
    } else if !status.contains("200") {
        return Err("GH API Error");
    }

    Ok(())
}