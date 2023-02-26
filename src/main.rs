mod git_data;
mod config_data;

extern crate core;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    PushPR {title: String, description: Option<String>},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::PushPR {title, description}) => {
            let _git_data = git_data::get_git_data();
            let _config_data = config_data::get_config_data();

            // call the GH API to create a PR
            // call the Slack API to post the PR link to the channel
        }
        None => {
            println!("Default subcommand");
        }
    }
}