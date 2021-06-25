use teloxide::types::{KeyboardButton, KeyboardMarkup, ReplyMarkup};

pub fn create_keyboard(buttons_texts: Vec<&str>) -> ReplyMarkup {
    let buttons: Vec<KeyboardButton> = buttons_texts
        .iter()
        .map(|b| KeyboardButton::new(b.to_string()))
        .collect();
    let keyboard = KeyboardMarkup::new(vec![buttons]).resize_keyboard(true);
    ReplyMarkup::Keyboard(keyboard)
}

pub fn dish_keyboard() -> ReplyMarkup {
    create_keyboard(vec!["Cпасибо!", "А можно чего другого?"])
}

pub fn menu_keyboard() -> ReplyMarkup {
    create_keyboard(vec!["Спасибо!", "Ладно, мне повезет!"])
}

pub fn start_keyboard() -> ReplyMarkup {
    create_keyboard(vec![
        "Чего бы мне поесть сегодня?",
        "А посоветуй-ка мне что-нибудь с...",
        "Огласите весь список!",
    ])
}
