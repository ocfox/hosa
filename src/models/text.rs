use reqwest::{self, header};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::config::Config;

pub type Messages = Vec<ChatMessage>;

pub struct ChatMessage {
    role: String,
    content: String,
}

pub struct Request {
    messages: Vec<Value>,
    headers: header::HeaderMap,
}

#[derive(Deserialize)]
pub struct Response {
    pub result: Result,
    pub success: bool,
    pub errors: Value,
    pub messages: Value,
}

#[derive(Deserialize)]
pub struct Result {
    pub response: String,
}

pub struct TextModel {
    pub url: String,
    pub token: String,
}

impl ChatMessage {
    pub fn new(role: String, content: String) -> ChatMessage {
        ChatMessage { role, content }
    }
}

impl Request {
    pub fn from(message: Messages, token: &String) -> Request {
        let mut headers = header::HeaderMap::new();

        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let messages = message
            .iter()
            .map(|m| {
                let mut message = Value::default();
                message["role"] = Value::String(m.role.to_owned());
                message["content"] = Value::String(m.content.to_owned());
                message
            })
            .collect::<Vec<Value>>();

        Request { messages, headers }
    }
}

impl TextModel {
    pub fn new(model: String, id: String, token: String) -> TextModel {
        let url = format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/ai/run/{}",
            id, model
        );
        TextModel { url, token }
    }

    pub fn from(config: &Config) -> TextModel {
        let model = config.text_model();
        let id = config.cloudflare.id.to_owned();
        let token = config.cloudflare.token.to_owned();

        TextModel::new(model, id, token)
    }

    pub fn default() -> TextModel {
        let config_path = std::env::args().nth(1).unwrap_or("config.toml".to_string());
        let config = Config::from(config_path);

        TextModel::from(&config)
    }

    pub async fn get_answer(
        &self,
        request: Request,
    ) -> core::result::Result<String, reqwest::Error> {
        let client = reqwest::Client::new();

        let json = json!({
            "messages": request.messages,
        });

        let response = client
            .post(self.url.to_owned())
            .headers(request.headers)
            .json(&json)
            .send()
            .await?;

        let response = response.json::<Response>().await?;

        Ok(response.result.response)
    }
}
