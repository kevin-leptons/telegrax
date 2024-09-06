use kit::create_services;

mod kit;

#[test]
fn send_message() {
    let (config, bot) = create_services();
    let content = "Hi there!";
    bot.send_message(config.telegram_chat_identity(), content)
        .unwrap();
}
