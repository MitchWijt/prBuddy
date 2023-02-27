use std::{str};
use std::env;

pub struct Config {
    pub slack_webhook_url: String,
    pub github_api_key: String
}

pub fn get_config_data() -> Result<Config, &'static str> {
    let slack_webhook_url = get_value_from_env("PR_BUDDY_SLACK_WEBHOOK_URL")?;
    let github_api_key = get_value_from_env("PR_BUDDY_GH_API_KEY")?;

    return Ok(Config{
        slack_webhook_url,
        github_api_key
    })
}

fn get_value_from_env(env_variable: &str) -> Result<String, &'static str> {
    let value = env::var(env_variable).expect(&*format!("ENV variable missing: {}", env_variable));
    return Ok(value)
}