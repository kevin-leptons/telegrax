use crate::kit::{create_context, sleep_after_sending_message};
use kix::Result;
use serial_test::serial;
use telegrax::bot::{LinkPreviewOptions, ParseMode, SendMessageOptions, SendPhotoOptions};
use telegrax::result::Error;

#[test]
#[serial]
fn send_message_with_default_options() -> Result<()> {
    let context = create_context();
    let chat_id = context.config().telegram_chat_identity();
    let content = "Hulk is here. Ahhh...";
    let options = SendMessageOptions::default();
    let result = context.bot().send_message(chat_id, content, options);
    sleep_after_sending_message();
    result?;
    Ok(())
}

#[test]
#[serial]
fn send_message_with_disabled_notification() -> Result<()> {
    let context = create_context();
    let chat_id = context.config().telegram_chat_identity();
    let content = "Hulk is here. Ahhh...";
    let options = SendMessageOptions {
        disable_notification: Some(true),
        ..Default::default()
    };
    let result = context.bot().send_message(chat_id, content, options);
    sleep_after_sending_message();
    result?;
    Ok(())
}

#[test]
#[serial]
fn send_message_with_disabled_link_preview() -> Result<()> {
    let context = create_context();
    let chat_id = context.config().telegram_chat_identity();
    let content = "Hulk is here. Ahhh...";
    let link_preview_options = LinkPreviewOptions {
        is_disabled: Some(true),
        ..Default::default()
    };
    let options = SendMessageOptions {
        link_preview_options: Some(link_preview_options),
        ..Default::default()
    };
    let result = context.bot().send_message(chat_id, content, options);
    result?;
    Ok(())
}

#[test]
#[serial]
fn send_message_with_parse_mode_markdown() -> Result<()> {
    let context = create_context();
    let chat_id = context.config().telegram_chat_identity();
    let content = "*Hulk is here*. Ahhh...";
    let options = SendMessageOptions {
        parse_mode: Some(ParseMode::Markdown),
        ..Default::default()
    };
    let result = context.bot().send_message(chat_id, content, options);
    sleep_after_sending_message();
    result?;
    Ok(())
}

#[test]
#[serial]
fn send_photo_with_caption() -> Result<()> {
    let context = create_context();
    let chat_id = context.config().telegram_chat_identity();
    let photo = "https://static1.srcdn.com/wordpress/wp-content/uploads/2023/02/hulk-in-avengers-age-of-ultron.jpg";
    let caption = "*Hulk is here*\n\nAhhhhh...";
    let options = SendPhotoOptions {
        caption: Some(caption.to_string()),
        parse_mode: Some(ParseMode::Markdown),
        ..Default::default()
    };
    let result = context.bot().send_photo(chat_id, photo, options);
    sleep_after_sending_message();
    result?;
    Ok(())
}

#[test]
#[serial]
fn send_message_error() {
    let context = create_context();
    let chat_id = "NOT_EXISTED_CHAT_ID";
    let content = "Hello there.";
    let options = SendMessageOptions::default();
    let descrition = match context.bot().send_message(chat_id, content, options) {
        Ok(_) => panic!("Send message does not return an error"),
        Err(e) => match e {
            Error::Endpoint { description } => description,
            _ => panic!("Send message return not matched error"),
        },
    };
    assert_eq!(descrition, "Bad Request: chat not found");
}
