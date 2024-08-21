use teloxide::payloads::{SendMessageSetters, SendPhotoSetters};
use teloxide::prelude::*;
use teloxide::types::ReplyParameters;
use teloxide::utils::command::BotCommands;
use teloxide::{self, types::InputFile};

use crate::config::Config;
use crate::models::{
    image::ImageModel,
    text::{self, TextModel},
};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Start a new chat with system message.")]
    New,
    #[command(description = "Display the current chat.", alias = "nl")]
    Chat,
    #[command(description = "Generate image from text.")]
    Image,
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::New => bot.send_message(msg.chat.id, "New chat started.").await?,
        Command::Chat => chat(bot.clone(), msg.clone()).await?,
        Command::Image => image(bot.clone(), msg.clone()).await?,
    };

    Ok(())
}

pub async fn chat(bot: Bot, msg: Message) -> ResponseResult<Message> {
    let config = Config::default();

    let reply = bot
        .send_message(msg.chat.id, config.wait_message())
        .reply_parameters(ReplyParameters::new(msg.id))
        .await?;

    let trimed = msg.text().unwrap().trim_start_matches("/chat").trim();

    let model = TextModel::default();
    let messages = vec![text::ChatMessage::new(
        "user".to_string(),
        trimed.to_string(),
    )];

    let request = text::Request::from(messages, &model.token);
    let response = model.get_answer(request).await.unwrap();

    bot.edit_message_text(msg.chat.id, reply.id, response).await
}

pub async fn image(bot: Bot, msg: Message) -> ResponseResult<Message> {
    let model = ImageModel::default();
    let prompt = msg.text().unwrap().trim_start_matches("/image").trim();
    let wait_image = Config::default().wait_image();
    let wait_message = bot
        .send_message(msg.chat.id, wait_image)
        .reply_parameters(ReplyParameters::new(msg.id))
        .await?;
    let image = model.request(prompt.to_string()).await.unwrap();

    bot.delete_message(msg.chat.id, wait_message.id).await?;

    bot.send_photo(msg.chat.id, InputFile::memory(image))
        .reply_parameters(ReplyParameters::new(msg.id))
        .await
}
