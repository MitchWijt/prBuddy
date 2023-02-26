mod git_data;
mod config_data;

extern crate core;

use clap::{Parser, Subcommand};
use serde_json::Value;
use serde_json::json;
use crate::git_data::GitData;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    PushPR {title: String, description: Option<String>},
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::PushPR {title, description}) => {
            let git_data = git_data::get_git_data().expect("error getting git data");
            let config_data = config_data::get_config_data().expect("error getting config");

            let response = call_gh_api(git_data, config_data.github_api_key).await?;
            let url = get_url_from_response(response);
            // call the GH API to create a PR
            // call the Slack API to post the PR link to the channel
        }
        None => {
            println!("Default subcommand");
        }
    }

    Ok(())
}

async fn call_gh_api(github_data: GitData, token: String, title: &String) -> reqwest::Result<String> {
    let test_body = json!({
        "title": title,
        "body": "New pull request",
        "head": github_data.branch,
        "base": github_data.main_branch
    });

    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/pulls", github_data.owner, github_data.repo_name);
    // let body = r#"{"title":"Amazing new feature","body":"Please pull these awesome changes in!","head":"integrate-github-api","base":"main"}"#;

    let resp = client.post(url)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", format!("{}", github_data.repo_name))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .body(test_body)
        .send()
        .await?;

    let response = resp.text().await?;
    Ok(response)
}

fn get_url_from_response(response: String) -> Result<String, &'static str> {
    let root: Value = serde_json::from_str(response.as_str())?;

    let url: Option<&str> = root.get("url")
        .and_then(|value| value.as_str());

    match url {
        Some(value) => Ok(String::from(value)),
        None => Err("Was not able to get URL from result")
    }
}