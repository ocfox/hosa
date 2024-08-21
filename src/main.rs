mod config;
mod models;
mod telegram;

use telegram::bot::{answer, Command};
use teloxide::repls::CommandReplExt;

#[tokio::main]
async fn main() {
    let path = std::env::args().nth(1).unwrap_or("config.toml".to_string());
    let config = config::Config::from(path);

    let bot = teloxide::Bot::new(&config.telegram_token);

    Command::repl(bot, answer).await;
}
