use crate::dialogue::dishes::Dish;
use rand::seq::SliceRandom;
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

pub fn choose_random_food(variants: Vec<Dish>) -> Dish {
    variants.choose(&mut rand::thread_rng()).unwrap().clone()
}

pub fn get_food_variants() -> Vec<Dish> {
    vec![
        Dish::new(
            "Хумус".to_string(),
            "Вкуснейшая ближневосточная закуска.".to_string(),
            vec!["нут", "тхина", "оливковое масло", "чеснок", "лимонный сок"],
        ),
        Dish::new(
            "Маджадра".to_string(),
            "Главное вегетарианское блюдо на свете".to_string(),
            vec!["рис", "чечевица", "лук"],
        ),
        Dish::new(
            "Шахи Панир".to_string(),
            "Нежный индийский сыр в томатном соусе".to_string(),
            vec!["панир", "помидоры", "перец"],
        ),
    ]
}

fn dish_keyboard() -> ReplyMarkup {
    create_two_button_keyboard("Cпасибо!", "А можно чего другого?")
}

fn menu_keyboard() -> ReplyMarkup {
    create_two_button_keyboard("Спасибо!", "Ладно, мне повезет!")
}
