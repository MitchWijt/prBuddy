use crate::config_data::Config;
use crate::GitData;
mod github_api;
mod gitlab_api;

pub async fn open_pull_request(git_data: &GitData, config_data: &Config, title: &String, description: &Option<String>) -> Result<String, &'static str> {
    let response = match &config_data.use_gitlab {
        Some(value) => {
            if value.eq(&String::from("True")) {
                gitlab_api::call_api(git_data, &config_data.gitlab_api_key, title, description).await?
            } else {
                github_api::call_api(git_data, &config_data.github_api_key, title, description).await?
            }
        },
        None => github_api::call_api(git_data, &config_data.github_api_key, title, description).await?
    };

    Ok(response.url)
}