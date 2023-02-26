mod git_data;
mod config_data;

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
async fn main() -> reqwest::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::PushPR {title, description}) => {
            let git_data = git_data::get_git_data().expect("error getting git data");
            let _config_data = config_data::get_config_data();

            call_gh_api(git_data).await?
            // call the GH API to create a PR
            // call the Slack API to post the PR link to the channel
        }
        None => {
            println!("Default subcommand");
        }
    }

    Ok(())
}

async fn call_gh_api(github_data: GitData) -> reqwest::Result<()> {
    let body = r#"{"title":"Amazing new feature","body":"Please pull these awesome changes in!","head":"feature-github-api","base":"main"}"#;

    let client = reqwest::Client::new();
    let resp = client.post("https://api.github.com/repos/MitchWijt/prBuddy/pulls")
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", "Bearer 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")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .body(body)
        .send()
        .await?;

    println!("{:?}", resp);

    Ok(())

    // let response = send(request.bo)
}