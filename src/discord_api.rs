use async_trait::async_trait;
use serde_json::json;
use crate::publisher::Publish;

pub struct DiscordApi<'a> {
    pr_url: &'a String,
    webhook_url: &'a String,
    title: &'a String,
}

impl DiscordApi<'_> {
    pub fn new<'a>(pr_url: &'a String, webhook_url: &'a Option<String>, title: &'a String) -> Result<DiscordApi<'a>, &'static str> {
        let webhook = webhook_url.as_ref().expect("Discord webhook URL was not found in ENV variables");

        Ok(DiscordApi {
            pr_url,
            webhook_url: webhook,
            title
        })
    }
}

#[async_trait]
impl Publish for DiscordApi<'_> {
    async fn publish(&self) -> Result<(), &'static str> {
        let body = json!({
            "username": "PrBuddy",
            "content": format!("PR: [{}]({})", self.title, self.pr_url),
        });

        let client = reqwest::Client::new();
        let resp = client.post(self.webhook_url)
            .header("Content-type", "application/json")
            .body(body.to_string())
            .send()
            .await
            .expect("Call to Discord API Failed");

        resp.text().await.expect("Getting response.text() failed");
        Ok(())
    }
}