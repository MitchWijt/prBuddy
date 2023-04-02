use serde_json::json;

pub async fn push_pr_to_slack(pr_url: &String, webhook_url: &Option<String>, title: &String) -> Result<String, &'static str> {
    let body = json!({
        "text": format!("PR: <{}|{}>", pr_url, title),
    });

    let webhook_url = webhook_url.as_ref().expect("Slack webhook URL was not found in ENV variables");

    let client = reqwest::Client::new();
    let resp = client.post(webhook_url)
        .header("Content-type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .expect("Call to Slack API Failed");

    let data = resp.text().await.expect("Getting response.text() failed");
    Ok(data)
}