use kit::create_context;
use telegrax::bot::{LinkPreviewOptions, ParseMode, SendMessageOptions, SendPhotoOptions};

mod kit;

#[test]
fn send_message_with_default_options() {
    let context = create_context();
    let chat_id = context.config().telegram_chat_identity();
    let content = "Hulk is here. Ahhh...";
    let options = SendMessageOptions::default();
    context
        .bot()
        .send_message(chat_id, content, options)
        .unwrap();
}

#[test]
fn send_message_with_disabled_notification() {
    let context = create_context();
    let chat_id = context.config().telegram_chat_identity();
    let content = "Hulk is here. Ahhh...";
    let options = SendMessageOptions {
        disable_notification: Some(true),
        ..Default::default()
    };
    context
        .bot()
        .send_message(chat_id, content, options)
        .unwrap();
}

#[test]
fn send_message_with_disabled_link_preview() {
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
    context
        .bot()
        .send_message(chat_id, content, options)
        .unwrap();
}

#[test]
fn send_message_with_parse_mode_markdown() {
    let context = create_context();
    let chat_id = context.config().telegram_chat_identity();
    let content = "*Hulk is here*. Ahhh...";
    let options = SendMessageOptions {
        parse_mode: Some(ParseMode::Markdown),
        ..Default::default()
    };
    context
        .bot()
        .send_message(chat_id, content, options)
        .unwrap();
}

#[test]
fn send_photo_with_caption() {
    let context = create_context();
    let chat_id = context.config().telegram_chat_identity();
    let photo = "https://static1.srcdn.com/wordpress/wp-content/uploads/2023/02/hulk-in-avengers-age-of-ultron.jpg";
    let caption = "*Hulk is here*\n\nAhhhhh...";
    let options = SendPhotoOptions {
        caption: Some(caption.to_string()),
        parse_mode: Some(ParseMode::Markdown),
        ..Default::default()
    };
    context.bot().send_photo(chat_id, photo, options).unwrap();
}
