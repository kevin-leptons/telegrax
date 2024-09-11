use kit::create_services;

mod kit;

#[test]
fn send_message() {
    let (config, bot) = create_services();
    let content = "Hi there!";
    bot.send_message(config.telegram_chat_identity(), content)
        .unwrap();
}

#[test]
fn send_image() {
    let (config, bot) = create_services();
    let image = "https://static1.srcdn.com/wordpress/wp-content/uploads/2023/02/hulk-in-avengers-age-of-ultron.jpg";
    let content = "Hulk is here";
    bot.send_image(config.telegram_chat_identity(), image, content)
        .unwrap();
}
