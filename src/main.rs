mod git_data;
mod config;
mod git_api;
mod github_api;
mod gitlab_api;
mod slack_api;
mod cli;
mod publisher;
mod discord_api;

extern crate core;

use crate::cli::Cli;

#[tokio::main]
async fn main() {
    match Cli::execute().await {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1)
        }
    }
}