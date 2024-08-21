use serde::Deserialize;
use std::{fs::File, io::Read};
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub telegram_token: String,
    pub cloudflare: Cloudflare,
    pub models: Option<Models>,
    pub bot: Option<Bot>,
}

#[derive(Debug, Deserialize)]
pub struct Cloudflare {
    pub id: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct Models {
    pub text: Option<String>,
    pub image_to_text: Option<String>,
    pub text_to_image: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Bot {
    pub wait_message: Option<String>,
    pub wait_image: Option<String>,
}

impl Config {
    pub fn from(path: String) -> Config {
        let mut file = File::open(path).expect("Config file not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read config file");

        let mut config: Config = toml::from_str(&contents).expect("Failed to parse config file");

        if config.models.is_none() {
            config.models = Some(Models {
                text: Some("@cf/meta/llama-3.1-8b-instruct".to_string()),
                text_to_image: Some("@cf/bytedance/stable-diffusion-xl-lightning".to_string()),
                image_to_text: Some("@cf/unum/uform-gen2-qwen-500m".to_string()),
            });
        }

        if config.bot.is_none() {
            config.bot = Some(Bot {
                wait_message: Some("Waiting for response...".to_string()),
                wait_image: Some("Waiting for drawing...".to_string()),
            });
        }

        config
    }

    pub fn default() -> Config {
        let config_path = std::env::args().nth(1).unwrap_or("config.toml".to_string());
        Config::from(config_path)
    }

    pub fn text_model(&self) -> String {
        self.models
            .as_ref()
            .unwrap()
            .text
            .as_ref()
            .unwrap()
            .to_owned()
    }

    // pub fn image_to_text_model(&self) -> String {
    //     self.models
    //         .as_ref()
    //         .unwrap()
    //         .image_to_text
    //         .as_ref()
    //         .unwrap()
    //         .to_owned()
    // }

    pub fn text_to_image_model(&self) -> String {
        self.models
            .as_ref()
            .unwrap()
            .text_to_image
            .as_ref()
            .unwrap()
            .to_owned()
    }

    pub fn wait_message(&self) -> String {
        self.bot
            .as_ref()
            .unwrap()
            .wait_message
            .as_ref()
            .unwrap()
            .to_owned()
    }

    pub fn wait_image(&self) -> String {
        self.bot
            .as_ref()
            .unwrap()
            .wait_image
            .as_ref()
            .unwrap()
            .to_owned()
    }
}
