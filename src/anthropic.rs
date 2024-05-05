use std::borrow::Cow;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;


#[derive(Serialize)]
struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Debug, Default)]
struct AnthropicResponse {
    pub _id: Option<String>,
    #[serde(default)]
    pub content: Vec<Content>,
    pub _model: Option<String>,
    pub _stop_reason: Option<String>,
    pub _stop_sequence: Option<String>,
    #[serde(default)]
    pub _usage: Usage,
}

#[derive(Deserialize, Debug, Default)]
struct Content {
    pub text: String,
    #[serde(rename = "type")]
    pub _content_type: String,
}

#[derive(Deserialize, Debug, Default)]
struct Usage {
    pub _input_tokens: i32,
    pub _output_tokens: i32,
}

#[tokio::main]
pub async fn anthropic(diff: Cow<str>) -> String {
    let api_key = "REPLACE_WITH_YOUR_API_KEY";
    let client = Client::new();

    let messages = vec![Message {
        role: "user".to_string(),
        content: diff.to_string(),
    }];

    let request_body = json!({
        "model": "claude-3-haiku-20240307",
        "max_tokens": 1024,
        "system": "Generate a commit message using the following changes. Limit your entire response to 72 characters.",
        "messages": messages,
    });

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("Content-Type", "application/json")
        .header("X-API-Key", api_key)
        .header("Anthropic-Version", "2023-06-01")
        .json(&request_body)
        .send()
        .await
        .expect("Failed to send request to Anthropic API");

    let response_body: AnthropicResponse = response
        .json()
        .await
        .expect("Failed to parse Anthropic API response");

    if let Some(content) = response_body.content.first() {
        content.text.clone()
    } else {
        String::new()
    }
}