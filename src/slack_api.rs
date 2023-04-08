use serde_json::json;

pub struct SlackApi<'a> {
    pr_url: &'a String,
    webhook_url: &'a String,
    title: &'a String,
}

impl SlackApi<'_> {
    pub fn new<'a>(pr_url: &'a String, webhook_url: &'a Option<String>, title: &'a String) -> Result<SlackApi<'a>, &'static str> {
        let webhook = webhook_url.as_ref().expect("Slack webhook URL was not found in ENV variables");

        Ok(SlackApi {
            pr_url,
            webhook_url: webhook,
            title
        })
    }

    pub async fn publish_pr(&self) -> Result<(), &'static str> {
        let body = json!({
            "text": format!("PR: <{}|{}>", self.pr_url, self.title),
        });


        let client = reqwest::Client::new();
        let resp = client.post(self.webhook_url)
            .header("Content-type", "application/json")
            .body(body.to_string())
            .send()
            .await
            .expect("Call to Slack API Failed");

        resp.text().await.expect("Getting response.text() failed");
        Ok(())
    }
}