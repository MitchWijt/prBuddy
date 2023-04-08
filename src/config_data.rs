use std::{str};
use std::env;

pub struct Config {
    pub slack_webhook_url: Option<String>,
    pub use_gitlab: Option<String>,
    pub github_api_key: Option<String>,
    pub gitlab_api_key: Option<String>,
}

impl Config {
    pub fn build() -> Self {
        let slack_webhook_url = Self::get_value_from_env("PR_BUDDY_SLACK_WEBHOOK_URL");
        let use_gitlab = Self::get_value_from_env("USE_GITLAB");
        let github_api_key = Self::get_value_from_env("PB_GITHUB_KEY");
        let gitlab_api_key = Self::get_value_from_env("PB_GITLAB_KEY");

        return Config{
            slack_webhook_url,
            use_gitlab,
            github_api_key,
            gitlab_api_key
        }
    }

    pub fn use_gitlab(&self) -> bool {
        match &self.use_gitlab {
            Some(v) => {
                return if v.eq(&String::from("True")) {
                    true
                } else {
                    false
                }
            },
            None => false
        }
    }

    fn get_value_from_env(env_variable: &str) -> Option<String> {
        match env::var(env_variable) {
            Ok(value) => Some(value),
            _other=> None
        }
    }
}