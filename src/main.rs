mod git_data;
mod config_data;
mod git_api;
mod slack_api;

extern crate core;

use clap::{Parser, Subcommand};
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
async fn main() -> Result<(), &'static str> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::PushPR {title, description}) => {
            let git_data = git_data::get_git_data().expect("error getting git data");
            let config_data = config_data::get_config_data().expect("error getting config");

            let pull_request_url = git_api::open_pull_request(
                &git_data,
                &config_data.github_api_key,
                title,
                description).await?;

            slack_api::push_pr_to_slack(&pull_request_url, &config_data.slack_webhook_url, title).await?;

            println!("Created PR ✨ {}", pull_request_url);
        }
        None => {
            println!("pr_buddy Err: Command was not found");
        }
    }

    Ok(())
}