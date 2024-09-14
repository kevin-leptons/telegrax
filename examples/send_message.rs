use telegrax::bot::{Bot, Configuration, SendMessageOptions};

fn main() {
    let bot_token = "PUT_VALUE_HERE";
    let chat_identity = "PUT_VALUE_HERE";
    let content = "A message from the bot.";
    let config = Configuration {
        token: bot_token.to_string(),
    };
    let bot = Bot::new(config);
    let options = SendMessageOptions::default();
    bot.send_message(chat_identity, content, options).unwrap();
}
