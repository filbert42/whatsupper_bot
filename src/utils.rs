use teloxide::types::{KeyboardButton, KeyboardMarkup, ReplyMarkup};

pub fn create_two_button_keyboard(
    first_button_text: &str,
    second_button_text: &str,
) -> ReplyMarkup {
    let firs_button = KeyboardButton::new(first_button_text.to_string());
    let second_button = KeyboardButton::new(second_button_text.to_string());
    let keyboard =
        KeyboardMarkup::new(vec![vec![firs_button, second_button]]).resize_keyboard(true);
    ReplyMarkup::Keyboard(keyboard)
}
