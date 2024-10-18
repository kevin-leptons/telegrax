//! Send message as a bot.

use crate::result::{Error, Result};
use serde::Serialize;

static ENDPOINT: &str = "https://api.telegram.org";

pub struct Bot {
    token: String,
}

pub struct Configuration {
    /// The token for bot authentication. For creating a new token, see
    /// [@BotFather](https://core.telegram.org/bots/tutorial#getting-ready).
    pub token: String,
}

#[derive(Default)]
pub struct SendMessageOptions {
    /// Mode for parsing entities in the message text.
    pub parse_mode: Option<ParseMode>,

    /// Link preview generation options for the message.
    pub link_preview_options: Option<LinkPreviewOptions>,

    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: Option<bool>,

    /// Unique identifier of the business connection on behalf of which the
    /// message will be sent.
    pub business_connection_id: Option<String>,

    /// Unique identifier for the target message thread (topic) of the forum;
    /// for forum supergroups only.
    pub message_thread_id: Option<u64>,

    /// Protects the contents of the sent message from forwarding and saving.
    pub protect_content: Option<bool>,
}

#[derive(Serialize, Default)]
pub struct LinkPreviewOptions {
    /// True, if the link preview is disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,

    /// URL to use for the link preview. If empty, then the first URL found in
    /// the message text will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// True, if the media in the link preview is supposed to be shrunk; ignored
    /// if the URL isn't explicitly specified or media size change isn't
    /// supported for the preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_small_media: Option<bool>,

    /// True, if the media in the link preview is supposed to be enlarged;
    /// ignored if the URL isn't explicitly specified or media size change isn't
    /// supported for the preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_large_media: Option<bool>,

    /// True, if the link preview must be shown above the message text;
    /// otherwise, the link preview will be shown below the message text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_above_text: Option<bool>,
}

#[derive(Serialize)]
pub enum ParseMode {
    /// See [Markdown](https://core.telegram.org/bots/api#markdown-style) for
    /// supported syntax.
    #[serde(rename = "Markdown")]
    Markdown,

    /// See [MarkdownV2](https://core.telegram.org/bots/api#markdownv2-style)
    /// for supported syntax.
    #[serde(rename = "MarkdownV2")]
    MarkdownV2,

    /// See [HTML](https://core.telegram.org/bots/api#html-style)
    /// for supported syntax.
    #[serde(rename = "HTML")]
    Html,
}

#[derive(Serialize)]
struct SendMessage {
    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername).
    pub chat_id: String,

    /// Text of the message to be sent, 1-4096 characters after entities
    /// parsing.
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}

#[derive(Serialize)]
struct SendPhoto {
    /// Unique identifier for the target chat or username of the target
    /// channel (in the format @channelusername).
    pub chat_id: String,

    /// Photo to send. Pass a file_id as String to send a photo that exists on
    /// the Telegram servers (recommended), pass an HTTP URL as a String for
    /// Telegram to get a photo from the Internet, or upload a new photo using
    /// multipart/form-data. The photo must be at most 10 MB in size. The
    /// photo's width and height must not exceed 10000 in total. Width and
    /// height ratio must be at most 20. More information on Sending Files.
    pub photo: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
}

#[derive(Default)]
pub struct SendPhotoOptions {
    /// Photo caption (may also be used when resending photos by file_id),
    /// 0-1024 characters after entities parsing.
    pub caption: Option<String>,

    /// Unique identifier of the business connection on behalf of which the
    /// message will be sent.
    pub business_connection_id: Option<String>,

    /// Unique identifier for the target message thread (topic) of the forum;
    /// for forum supergroups only.
    pub message_thread_id: Option<u64>,

    /// Mode for parsing entities in the photo caption. See formatting options
    /// for more details.
    pub parse_mode: Option<ParseMode>,

    /// Pass True, if the caption must be shown above the message media.
    pub show_caption_above_media: Option<bool>,

    /// Pass True if the photo needs to be covered with a spoiler animation.
    pub has_spoiler: Option<bool>,

    /// Sends the message silently. Users will receive a notification with no
    /// sound..
    pub disable_notification: Option<bool>,

    /// Protects the contents of the sent message from forwarding and saving.
    pub protect_content: Option<bool>,

    /// Unique identifier of the message effect to be added to the message; for
    /// private chats only.
    pub message_effect_id: Option<String>,
}

impl Bot {
    pub fn new(config: Configuration) -> Self {
        Self {
            token: config.token,
        }
    }

    /// # Arguments
    ///
    /// `chat_identity`. Unique identifier for the target chat or username of
    /// the target channel (in the format @channelusername).
    ///
    /// `text`. Content to be sent. It must be not empty and less than 4096
    ///  characters after parsing. The parsing mode is specified by
    ///  [SendMessageOptions::parse_mode].
    pub fn send_message(
        &self,
        chat_identity: &str,
        text: &str,
        options: SendMessageOptions,
    ) -> Result<()> {
        let url = format!("{}/bot{}/sendMessage", ENDPOINT, self.token);
        let message = SendMessage {
            chat_id: chat_identity.to_string(),
            text: text.to_string(),
            business_connection_id: options.business_connection_id,
            disable_notification: options.disable_notification,
            link_preview_options: options.link_preview_options,
            message_thread_id: options.message_thread_id,
            parse_mode: options.parse_mode,
            protect_content: options.protect_content,
        };
        ureq::post(&url)
            .set("accept", "application/json")
            .send_json(message)
            .map_err(Error::from_http_api_error)?;
        Ok(())
    }

    /// # Arguments
    ///
    /// `chat_identity`. Unique identifier for the target chat or username of
    /// the target channel (in the format @channelusername).
    ///
    /// `photo`. Pass a *file_id* to send a photo that exists on the Telegram
    /// servers (recommended). Pass an HTTP URL to get a photo from the
    /// Internet. Or upload a new photo using *multipart/form-data*. The photo
    /// must be at most 10 MB in size. The photo's width and height must not
    /// exceed 10000 in total. Width and height ratio must be at most 20. More
    /// information on [Sending
    /// Files](https://core.telegram.org/bots/api#sending-files).
    pub fn send_photo(
        &self,
        chat_identity: &str,
        photo: &str,
        options: SendPhotoOptions,
    ) -> Result<()> {
        let endpoint = format!("{}/bot{}/sendPhoto", ENDPOINT, self.token);
        let message = SendPhoto {
            chat_id: chat_identity.to_string(),
            photo: photo.to_string(),
            business_connection_id: options.business_connection_id,
            caption: options.caption,
            disable_notification: options.disable_notification,
            has_spoiler: options.has_spoiler,
            message_effect_id: options.message_effect_id,
            message_thread_id: options.message_thread_id,
            parse_mode: options.parse_mode,
            protect_content: options.protect_content,
            show_caption_above_media: options.show_caption_above_media,
        };
        ureq::post(&endpoint)
            .set("accept", "application/json")
            .send_json(message)
            .map_err(Error::from_http_api_error)?;
        Ok(())
    }
}
