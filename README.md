# Welcome to PrBuddy!
![](./assets/logo.png)

This small convenient CLI is made so that you don't have to go 
through the daily hassle any more of continuously.
copying PR titles and URLs but have this done for you with one single command :-)

## How to use it: 

### Using Homebrew
run the following command to install with homebrew:
- ```brew tap MitchWijt/prbuddy && brew install prbuddy```

### Manually
2. Clone or fork this repo
3. make sure you have installed cargo
4. run `cargo build --release`
5. take the pr_buddy binary from `./target/release`
6. place this binary in your machine

### Config variables
prBuddy uses 2 config variables in order to work correctly. 
These are your Slack webhook URL and your GitHub personal access token.
Make sure to export these in your machine, for example in the `.zshrc` file.

here is an example for the ENV variables. Both of these variables need to be included:

```
PR_BUDDY_SLACK_WEBHOOK_URL=<YOUR WEBHOOK URL>
PR_BUDDY_GH_API_KEY=<GITHUB PERSONAL ACCESS TOKEN>
```

You're all set :-)

now you can push your changes to a branch and open a PR with the following command `pr_buddy push-pr <PR_TITLE>`
