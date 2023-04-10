use async_trait::async_trait;
use serde_json::json;
use serde::{Deserialize, Serialize};
use crate::git_api::{ApiPayload, GitApi, GitApiResponse};
use crate::git_data::GitData;

pub struct GitHubApi<'a> {
    git_data: &'a GitData,
    token: &'a String,
    title: &'a String,
    description: String
}

#[derive(Serialize, Deserialize, Debug)]
struct PullRequestCreationResponse {
    html_url: String,
}

impl GitHubApi<'_> {
    pub fn new<'a>(git_data: &'a GitData, token: &'a Option<String>, title: &'a String, description: &'a Option<String>) -> Result<GitHubApi<'a>, &'static str> {
        let desc = match description {
            Some(v) => String::from(v),
            None => String::from("New pull request")
        };

        let access_token = token.as_ref().expect("GitHub API token not found in ENV variables");

        Ok(GitHubApi {
            git_data,
            token: access_token,
            title,
            description: String::from(desc)
        })
    }

    fn handle_api_error(status: &String, body: &String) ->  Result<(), &'static str> {
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
}

#[async_trait]
impl GitApi for GitHubApi<'_> {
    async fn open_pull_request(&self) -> Result<GitApiResponse, &'static str> {
        let payload = self.call_api().await.expect("ca");

        let response_body: PullRequestCreationResponse = serde_json::from_str(payload.body.as_str())
            .expect("JSON validation failed");

        Ok(GitApiResponse {
            url: response_body.html_url
        })
    }

    async fn call_api(&self) -> Result<ApiPayload, &'static str> {
        let body = json!({
            "title": self.title,
            "body": self.description,
            "head": self.git_data.branch,
            "base": self.git_data.main_branch
        });

        let client = reqwest::Client::new();
        let url = format!("https://api.github.com/repos/{}/{}/pulls", self.git_data.owner, self.git_data.repo_name);

        let resp = client.post(url)
            .header("Accept", "application/vnd.github+json")
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", format!("{}", self.git_data.repo_name))
            .header("X-GitHub-Api-Version", "2022-11-28")
            .body(body.to_string())
            .send()
            .await
            .expect("Call to GH API Failed");

        let status = resp.status().to_string();
        let body = resp.text().await.expect("Getting response.text() failed");

        GitHubApi::handle_api_error(&status, &body)?;

        Ok(ApiPayload {status, body})
    }
}