use std::fs::{File, read_to_string};
use std::{str};
use std::io::{BufRead, BufReader};

pub struct Config {
    pub slack_webhook_url: String,
    pub github_api_key: String
}

pub fn get_config_data() -> Result<Config, &'static str> {
    let slack_webhook_url = get_value_from_config_file("SLACK_WEBHOOK_URL")?;
    let github_api_key = get_value_from_config_file("GITHUB_API_KEY")?;

    return Ok(Config{
        slack_webhook_url,
        github_api_key
    })
}

fn get_value_from_config_file(env: &str) -> Result<String, &'static str> {
    let file = File::open(".prBuddy").expect("Failed to open config file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let result = line.expect("Failed to read line");

        let split = result.split("=");
        let vector: Vec<&str> = split.collect();

        let config_variable = *vector.get(0).expect("Failed to grab config var");
        let config_value = *vector.get(1).expect("Failed to grab config var value");

        if config_variable == env {
            return Ok(String::from(config_value))
        }
    }

    Err("Failed to read config value")
}