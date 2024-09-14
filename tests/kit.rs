use serde::Deserialize;
use std::{path::PathBuf, str::FromStr};
use telegrax::bot::Bot;

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Configuration {
    telegram_bot_token: String,
    telegram_chat_identity: String,
}

pub struct Context {
    config: Configuration,
    bot: Bot,
}

impl Configuration {
    pub fn load() -> Self {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let files = [PathBuf::from_str(&crate_path)
            .unwrap()
            .join("config.test.json")];
        let (config, _) = luci::read_jsonc(files.iter()).unwrap();
        config
    }

    pub fn telegram_bot_token(&self) -> &str {
        &self.telegram_bot_token
    }

    pub fn telegram_chat_identity(&self) -> &str {
        &self.telegram_chat_identity
    }
}

pub fn create_context() -> Context {
    let config = Configuration::load();
    let bot_config = telegrax::bot::Configuration {
        token: config.telegram_bot_token().to_string(),
    };
    let bot = Bot::new(bot_config);
    Context { config, bot }
}

impl Context {
    pub fn config(&self) -> &Configuration {
        &self.config
    }

    pub fn bot(&self) -> &Bot {
        &self.bot
    }
}
