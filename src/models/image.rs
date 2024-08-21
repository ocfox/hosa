use bytes::Bytes;
use reqwest;

use crate::config::Config;

pub struct ImageModel {
    pub url: String,
    pub token: String,
}

impl ImageModel {
    pub fn from(config: &Config) -> ImageModel {
        let url = format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/ai/run/{}",
            config.cloudflare.id,
            config.text_to_image_model()
        );
        ImageModel {
            url,
            token: config.cloudflare.token.to_owned(),
        }
    }

    pub fn default() -> ImageModel {
        let config_path = std::env::args().nth(1).unwrap_or("config.toml".to_string());
        let config = Config::from(config_path);

        ImageModel::from(&config)
    }

    pub async fn request(&self, prompt: String) -> Result<Bytes, reqwest::Error> {
        let client = reqwest::Client::new();
        let response = client
            .post(&self.url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&serde_json::json!({ "prompt": prompt }))
            .send()
            .await?;

        let response = response.error_for_status()?;
        let image = response.bytes().await?;

        Ok(image)
    }
}
