mod git_data;
mod config_data;
mod git_api;

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
async fn main() -> Result<(), &'static str> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::PushPR {title, description}) => {
            let git_data = git_data::get_git_data().expect("error getting git data");
            let config_data = config_data::get_config_data().expect("error getting config");

            let pull_request_url = git_api::open_pull_request(&git_data, &config_data.github_api_key, title)
                .await?;

            println!("{}", pull_request_url);

            // call the GH API to create a PR
            // call the Slack API to post the PR link to the channel
        }
        None => {
            println!("Default subcommand");
        }
    }

    Ok(())
}