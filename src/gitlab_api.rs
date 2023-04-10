use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::git_api::{ApiPayload, GitApi, GitApiResponse};
use crate::git_data::GitData;

pub struct GitLabApi<'a> {
    git_data: &'a GitData,
    token: &'a String,
    title: &'a String,
    description: String
}

#[derive(Serialize, Deserialize, Debug)]
struct GitlabProject {
    id: i32,
    name: String,
    path: String,
    default_branch: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MergeRequestResponse {
    project_id: i32,
    title: String,
    description: String,
    web_url: String,
}

impl GitLabApi<'_> {
    pub fn new<'a>(git_data: &'a GitData, token: &'a Option<String>, title: &'a String, description: &'a Option<String>) -> Result<GitLabApi<'a>, &'static str> {
        let desc = match description {
            Some(v) => String::from(v),
            None => String::from("New pull request")
        };

        let access_token = token.as_ref().expect("GitLab API token not found in ENV variables");

        Ok(GitLabApi {
            git_data,
            token: access_token,
            title,
            description: String::from(desc)
        })
    }

    async fn list_projects(&self) -> Result<Vec<GitlabProject>, &'static str> {
        let client = reqwest::Client::new();
        let url = "https://gitlab.com/api/v4/projects?owned=true";

        let resp = client.get(url)
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", String::from(self.token)))
            .send()
            .await
            .expect("Call to GitLab API Failed");

        let status = resp.status().to_string();
        if !status.eq("200 OK") {
            return Err("Listing projects API call failed")
        }

        let projects: Vec<GitlabProject> = serde_json::from_str(&*resp.text().await.expect("Getting response.text() failed"))
            .expect("JSON validation failed");

        Ok(projects)
    }

    fn get_project<'a>(&self, projects: &'a Vec<GitlabProject>) -> Result<&'a GitlabProject, &'static str> {
        for project in projects.iter() {
            if project.name.eq(&self.git_data.repo_name) {
                return Ok(project)
            }
        };

        return Err("Project does not exist in GitLab");
    }

    async fn open_merge_request(&self, project: &GitlabProject) -> Result<ApiPayload, &'static str> {
        let client = reqwest::Client::new();
        let url = format!("https://gitlab.com/api/v4/projects/{}/merge_requests", &project.id);

        let body = json!({
        "id": &project.id,
        "title": self.title,
        "description": &self.description,
        "source_branch": self.git_data.branch,
        "target_branch": self.git_data.main_branch
    });

        let resp = client.post(url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", String::from(self.token)))
            .body(body.to_string())
            .send()
            .await
            .expect("Creating merge request failed");

        let status = resp.status().to_string();
        let body = resp.text().await.expect("Getting response.text() failed");

        if !status.eq("201 Created") {
            return Err("Creating merge request API call failed")
        }

        Ok(ApiPayload {status, body})
    }
}

#[async_trait]
impl GitApi for GitLabApi<'_> {
    async fn open_pull_request(&self) -> Result<GitApiResponse, &'static str> {
        let api_response = self.call_api().await?;

        let response_body: MergeRequestResponse = serde_json::from_str(api_response.body.as_str())
            .expect("JSON validation failed");

        Ok(GitApiResponse {
            url: response_body.web_url
        })
    }

    async fn call_api(&self) -> Result<ApiPayload, &'static str> {
        let projects = self.list_projects().await?;
        let project = self.get_project(&projects)?;
        let merge_request = self.open_merge_request(&project).await?;

        Ok(merge_request)
    }
}