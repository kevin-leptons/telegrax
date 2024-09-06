use telegrax::bot::{Bot, Configuration};

fn main() {
    let bot_token = "PUT_VALUE_HERE";
    let chat_identity = "PUT_VALUE_HERE";
    let message_content = "A message from the bot.";
    let config = Configuration {
        token: bot_token.to_string(),
    };
    let bot = Bot::new(config);
    bot.send_message(chat_identity, message_content).unwrap();
}
