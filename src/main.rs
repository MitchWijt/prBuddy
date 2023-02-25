extern crate core;

use std::str;
use std::process::Command;
use clap::{Parser, Subcommand};
use shutil::pipe;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

struct GitCommandArgs<'a> {
    branch: Vec<Vec<&'a str>>,
    main_branch: Vec<Vec<&'a str>>,
    remote_url: Vec<Vec<&'a str>>,
}

struct GitData {
    branch: &'static str,
    main_branch: &'static str,
    repo_name: &'static str,
    owner: &'static str,
}

#[derive(Subcommand)]
enum Commands {
    PushPR {title: String, description: Option<String>},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::PushPR {title, description}) => {
            let git_command_args = get_git_command_args();
            let branch_name = match get_command_output(git_command_args.remote_url, "git") {
                Ok(name) => println!("{}", name),
                Err(e) => panic!("{}", e),
            };


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

fn get_git_command_args<'a>() -> GitCommandArgs<'a> {
    return GitCommandArgs {
        branch: vec![vec!["git", "rev-parse", "--abbrev-ref", "HEAD"]],
        main_branch: vec![vec!["git", "symbolic-ref", "refs/remotes/origin/HEAD"], vec!["sed", "s@^refs/remotes/origin/@@"]],
        remote_url: vec![vec!["git", "config", "--get", "remote.origin.url"]],
    }
}

fn get_command_output(args: Vec<Vec<&str>>, program_name: &str) -> Result<String, &'static str> {
    let output = pipe(args);
    match output {
        Ok(output_str) => Ok(output_str.replace("\n", "")),
        Err(_e) => Err("Failed to execute command")
    }
}