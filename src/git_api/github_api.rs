use serde_json::json;
use serde::{Deserialize, Serialize};
use crate::git_structs::PullRequestResponse;
use crate::GitData;

#[derive(Serialize, Deserialize, Debug)]
struct PullRequestCreationResponse {
    html_url: String,
}

pub async fn call_api(github_data: &GitData, token: &Option<String>, title: &String, description: &Option<String>) -> Result<PullRequestResponse, &'static str> {
    let desc = match description {
        Some(v) => String::from(v),
        None => String::from("New pull request")
    };

    let api_token = token.as_ref().expect("GitHub API token not found in ENV variables");

    let body = json!({
        "title": title,
        "body": &desc,
        "head": github_data.branch,
        "base": github_data.main_branch
    });

    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/pulls", github_data.owner, github_data.repo_name);

    let resp = client.post(url)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", api_token))
        .header("User-Agent", format!("{}", github_data.repo_name))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .body(body.to_string())
        .send()
        .await
        .expect("Call to GH API Failed");

    let status = resp.status().to_string();
    if !status.eq(&String::from("200.OK")) {
        return Err("Call to GH API Failed. Status != 200")
    }

    let body = resp.text().await.expect("Getting response.text() failed");
    handle_api_error(&status, &body).await?;

    let pull_request: PullRequestCreationResponse = serde_json::from_str(body.as_str())
        .expect("JSON validation failed");

    Ok(PullRequestResponse {
        url: pull_request.html_url
    })
}

async fn handle_api_error(status: &String, body: &String) -> Result<(), &'static str> {
    if status.contains("422") {
        if body.contains("head") && body.contains("invalid") {
            return Err("Branch name does not exist on remote. Make sure to push changes first");
        } else if body.contains("A pull request already exists") {
            return Err("Pull Request already exists");
        }
    } else if status.contains("201") == false {
        return Err("GH API Error");
    }

    Ok(())
}