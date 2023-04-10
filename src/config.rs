use std::{str};
use std::env;

pub struct Config {
    pub slack_webhook_url: Option<String>,
    pub discord_webhook_url: Option<String>,
    pub github_api_key: Option<String>,
    pub gitlab_api_key: Option<String>,
}

impl Config {
    pub fn build() -> Self {
        let slack_webhook_url = Self::get_value_from_env("PB_SLACK_WEBHOOK_URL");
        let discord_webhook_url = Self::get_value_from_env("PB_DISCORD_WEBHOOK_URL");
        let github_api_key = Self::get_value_from_env("PB_GITHUB_KEY");
        let gitlab_api_key = Self::get_value_from_env("PB_GITLAB_KEY");

        return Config{
            slack_webhook_url,
            discord_webhook_url,
            github_api_key,
            gitlab_api_key
        }
    }

    pub fn use_gitlab(&self) -> bool {
        match &self.gitlab_api_key {
            Some(_) => {
                return true
            },
            None => false
        }
    }

    pub fn use_discord(&self) -> bool {
        match &self.discord_webhook_url {
            Some(_) => {
                return true
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