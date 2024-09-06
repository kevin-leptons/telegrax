use crate::error::Error;
use crate::result::Result;
use ureq::json;

static ENDPOINT: &str = "https://api.telegram.org";

pub struct Bot {
    token: String,
}

pub struct Configuration {
    pub token: String,
}

impl Bot {
    pub fn new(config: Configuration) -> Self {
        Self {
            token: config.token,
        }
    }

    /// Send a text message to a chat.
    pub fn send_message(&self, chat_identity: &str, content: &str) -> Result<()> {
        let url = format!("{}/bot{}/sendMessage", ENDPOINT, self.token);
        let request_body = json!({
            "chat_id": chat_identity,
            "text": content
        });
        ureq::post(&url)
            .set("content-type", "application/json")
            .set("accept", "application/json")
            .send_json(request_body)
            .map_err(Error::from_http_api_error)?;
        Ok(())
    }
}
