use crate::GitData;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::git_structs::PullRequestResponse;

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

pub async fn call_api(git_data: &GitData, token: &Option<String>, title: &String, description: &Option<String>) -> Result<PullRequestResponse, &'static str> {
    let desc = match description {
        Some(v) => String::from(v),
        None => String::from("New pull request")
    };

    let access_token = token.as_ref().expect("GitLab API token not found in ENV variables");

    let projects = list_all_projects(&access_token).await?;
    let project = get_project(&projects, &git_data.repo_name)?;
    let merge_request = open_merge_request(&project, git_data, &access_token, title, &desc).await?;


    Ok(PullRequestResponse {
        url: merge_request.web_url
    })
}

async fn list_all_projects(access_token: &String) -> Result<Vec<GitlabProject>, &'static str> {
    let client = reqwest::Client::new();
    let url = "https://gitlab.com/api/v4/projects?owned=true";

    let resp = client.get(url)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", String::from(access_token)))
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

fn get_project<'a>(projects: &'a Vec<GitlabProject>, repo_name: &String) -> Result<&'a GitlabProject, &'static str> {
    for project in projects.iter() {
        if project.name.eq(repo_name) {
            return Ok(project)
        }
    };

    return Err("Project does not exist in GitLab");
}

async fn open_merge_request(project: &GitlabProject, git_data: &GitData, access_token: &String, title: &String, description: &String) -> Result<MergeRequestResponse, &'static str> {
    let client = reqwest::Client::new();
    let url = format!("https://gitlab.com/api/v4/projects/{}/merge_requests", &project.id);

    let body = json!({
        "id": &project.id,
        "title": title,
        "description": &description,
        "source_branch": git_data.branch,
        "target_branch": git_data.main_branch
    });

    let resp = client.post(url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", String::from(access_token)))
        .body(body.to_string())
        .send()
        .await
        .expect("Creating merge request failed");

    let status = resp.status().to_string();
    let body = resp.text().await.expect("Getting response.text() failed");

    if !status.eq("201 Created") {
        return Err("Creating merge request API call failed")
    }

    let merge_request: MergeRequestResponse = serde_json::from_str(body.as_str())
        .expect("JSON validation failed");

    Ok(merge_request)
}