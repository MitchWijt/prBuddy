use clap::{Parser, Subcommand};
use crate::config::Config;
use crate::discord_api::DiscordApi;
use crate::git_api::GitApi;
use crate::git_data::GitData;
use crate::github_api::GitHubApi;
use crate::gitlab_api::GitLabApi;
use crate::publisher::Publish;
use crate::slack_api::SlackApi;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    PushPR {
        title: String,
        description: Option<String>,

        #[clap(long)]
        no_publish: bool,
    },
}

impl Cli {
    pub async fn execute() -> Result<(), &'static str> {
        let cli = Cli::parse();

        match &cli.command {
            Some(Commands::PushPR {title, description, no_publish}) => {
                let git_data = GitData::build().expect("error getting git data");
                let config =  Config::build();

                let response = if config.use_gitlab() {
                    let gitlab_api = GitLabApi::new(
                        &git_data,
                        &config.gitlab_api_key,
                        title,
                        description)?;

                    gitlab_api.open_pull_request().await?
                } else {
                    let github_api = GitHubApi::new(
                        &git_data,
                        &config.github_api_key,
                        title,
                        description)?;

                    github_api.open_pull_request().await?
                };

                if *no_publish == false {
                    if config.use_discord() {
                        let discord_api = DiscordApi::new(&response.url, &config.discord_webhook_url, &title)?;
                        discord_api.publish().await?;
                    } else {
                        let slack_api = SlackApi::new(&response.url, &config.slack_webhook_url, &title)?;
                        slack_api.publish().await?;
                    }
                }

                println!("Created PR ✨ {}", response.url);
            }
            None => {
                println!("pr_buddy Err: Command was not found");
            }
        }

        Ok(())
    }
}