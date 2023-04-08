mod git_data;
mod config_data;
mod git_api;
mod github_api;
mod gitlab_api;
mod slack_api;

extern crate core;

use clap::{Parser, Subcommand};
use crate::config_data::Config;
use crate::git_api::GitApi;
use crate::git_data::GitData;
use crate::github_api::GitHubApi;
use crate::gitlab_api::GitLabApi;
use crate::slack_api::SlackApi;

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
async fn main() -> Result<(), &'static str> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::PushPR {title, description}) => {
            let git_data = GitData::build().expect("error getting git data");
            let config_data =  Config::build();

            let response = if config_data.use_gitlab() {
                let gitlab_api = GitLabApi::new(
                    &git_data,
                    &config_data.github_api_key,
                    title,
                    description)?;

                gitlab_api.open_pull_request().await?
            } else {
                let github_api = GitHubApi::new(
                    &git_data,
                    &config_data.github_api_key,
                    title,
                    description)?;

                github_api.open_pull_request().await?
            };

            let slack_api = SlackApi::new(&response.url, &config_data.slack_webhook_url, &title)?;
            slack_api.publish_pr().await?;

            println!("Created PR ✨ {}", response.url);
        }
        None => {
            println!("pr_buddy Err: Command was not found");
        }
    }

    Ok(())
}