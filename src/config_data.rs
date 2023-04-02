use std::{str};
use std::env;

pub struct Config {
    pub slack_webhook_url: Option<String>,
    pub use_gitlab: Option<String>,
    pub github_api_key: Option<String>,
    pub gitlab_api_key: Option<String>,
}

pub fn get_config_data() -> Config {
    let slack_webhook_url = get_value_from_env("PR_BUDDY_SLACK_WEBHOOK_URL");
    let use_gitlab = get_value_from_env("USE_GITLAB");
    let github_api_key = get_value_from_env("PB_GITHUB_KEY");
    let gitlab_api_key = get_value_from_env("PB_GITLAB_KEY");

    return Config{
        slack_webhook_url,
        use_gitlab,
        github_api_key,
        gitlab_api_key
    }
}

fn get_value_from_env(env_variable: &str) -> Option<String> {
    match env::var(env_variable) {
        Ok(value) => Some(value),
        _other=> None
    }
}