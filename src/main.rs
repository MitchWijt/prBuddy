mod git_data;

extern crate core;

use std::str;
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
            let git_data = git_data::get_git_data();

            // read out the .prHelper/.config file to get the ENV variables
            // call the GH API to create a PR
            // call the Slack API to post the PR link to the channel
        }
        None => {
            println!("Default subcommand");
        }
    }
}