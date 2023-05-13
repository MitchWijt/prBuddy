# Welcome to PrBuddy!
![](./assets/logo.png)

This small convenient CLI is made so that you don't have to go 
through the daily hassle any more of continuously.
copying PR titles and URLs but have this done for you with one single command :-)

## How to use it: 

### Using Homebrew
run the following command to install with homebrew:
- ```brew tap MitchWijt/prbuddy && brew install prbuddy```

### Using Cargo
run the following command to install with cargo:
- ```cargo install pr_buddy```

### Manually
1. Clone or fork this repo
2. make sure you have installed cargo
3. run `cargo build --release`
4. take the pr_buddy binary from `./target/release`
5. place this binary in your machine

### Config variables
prBuddy uses multiple config variables in order to work correctly.
Make sure to export these in your machine, for example in the `.zshrc` file.

Currently we support both GitHub and GitLab:

*GitHub*:
```
PB_SLACK_WEBHOOK_URL=<YOUR WEBHOOK URL>
PB_GITHUB_KEY=<GITHUB PERSONAL ACCESS TOKEN>
```

*GitLab*:
```
PB_SLACK_WEBHOOK_URL=<YOUR WEBHOOK URL>
PB_GITLAB_KEY=<GITHUB PERSONAL ACCESS TOKEN>
```

Using Discord instead of Slack? No problemo! 
Just swap the `PB_SLACK_WEBHOOK_URL` for the `PB_DISCORD_WEBHOOK_URL`:
```
PB_DISCORD_WEBHOOK_URL=<YOUR WEBHOOK URL>
```

### Commands
- `pr_buddy push-pr <PR_TITLE>`
- `pr_buddy push-pr <PR_TITLE> <PR_DESCRIPTION>`
- `pr_buddy push-pr --no-publish <PR_TITLE> <PR_DESCRIPTION>`

You're all set :-) Happy coding!
