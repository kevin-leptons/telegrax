//! This program sends a text message to a chat without notification.
//!
//! To run the program, export the following environment variables:
//!
//! ```bash
//! export BOT_TOKEN="PUT_VALUE_HERE"
//! export CHAT_IDENTITY="PUT_VALUE_HERE"
//! ```

use std::env;
use std::error::Error;

use telegrax::bot::{Bot, Configuration, SendMessageOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let config = Configuration {
        token: get_environment_variable("BOT_TOKEN")?,
    };
    let bot = Bot::new(config);
    let chat_identity = get_environment_variable("CHAT_IDENTITY")?;
    let content = "This is a text message.";
    let options = SendMessageOptions {
        disable_notification: Some(true),
        ..Default::default()
    };
    bot.send_message(&chat_identity, content, options)?;
    Ok(())
}

fn get_environment_variable(key: &str) -> Result<String, Box<dyn Error>> {
    let value = env::var(key).or(Err(format!("Can not get environment variable {key}")))?;
    Ok(value)
}
