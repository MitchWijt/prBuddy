use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

struct GitData {
    branch: String,
    main_branch: String,
    repo_name: String,
    owner: String,
}

#[derive(Subcommand)]
enum Commands {
    PushPR {title: String, description: Option<String>},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::PushPR {title, description}) => {
            // read out of the .git folder to get branch name, repo name and HEAD name
            // read out the .prHelper/.config file to get the ENV variables
            // call the GH API to create a PR
            // call the Slack API to post the PR link to the channel
        }
        None => {
            println!("Default subcommand");
        }
    }
}