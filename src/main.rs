use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    PushPR {title: String, description: Option<String>},
    SetConfig {name: String, value: String},
    DelConfig {name: String}
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
        Some(Commands::SetConfig { name, value}) => {
            // config package that adds these values to the .prHelper/.config file
            println!("'myapp add' was used, name is: {:?}", name);
            println!("'myapp add' was used, value is: {:?}", value)
        }
        Some(Commands::DelConfig {name}) => {
            // config package that removes these values from the .prHelper/.config file
            println!("Delete config item with name: {:?}", name);
        }
        None => {
            println!("Default subcommand");
        }
    }
}
