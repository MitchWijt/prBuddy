# Welcome to PrBuddy!
![](./assets/logo.png)

This small convenient CLI is made so you don't have to go 
through the daily hassle anymore to continuously 
copy PR titles and URls's but have this one for you with one single command :-)

## How to use it: 
1. Clone or fork this repo
2. make sure you have installed cargo
3. run `cargo build --release`
4. take the pr_buddy binary from `./target/release`
5. place this binary in your own repo
6. create a .prBuddy config file
7. add this file to you .gitIgnore

You're all set :-) 

now you can push your changes to a branch and open a PR with the following command `./pr_buddy push-pr <PR_TITLE>`

here is an example for the .prBuddy config file. Both of these variables need to be included: 

```
SLACK_WEBHOOK_URL=<YOUR WEBHOOK URL>
GITHUB_API_KEY=<GITHUB PERSONAL ACCESS TOKEN>
```