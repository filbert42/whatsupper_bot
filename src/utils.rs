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

pub fn choose_random_food(variants: &Vec<Dish>) -> Option<&Dish> {
    variants.choose(&mut rand::thread_rng())
}

pub fn get_food_variants() -> Vec<Dish> {
    vec![
        Dish::new(
            "Хумус",
            "Вкуснейшая ближневосточная закуска.",
            vec!["нут", "тхина", "оливковое масло", "чеснок", "лимонный сок"],
        ),
        Dish::new(
            "Маджадра",
            "Главное вегетарианское блюдо на свете",
            vec!["рис", "чечевица", "лук"],
        ),
        Dish::new(
            "Шахи Панир",
            "Нежный индийский сыр в томатном соусе",
            vec!["панир", "помидоры", "перец"],
        ),
    ]
}

pub fn dish_suggestion_text(dish: &Dish) -> String {
    format!(
        "Я предлагаю тебе отведать сегодня:\n{0}",
        dish.format_to_string()
    )
}

pub fn dish_keyboard() -> ReplyMarkup {
    create_two_button_keyboard("Cпасибо!", "А можно чего другого?")
}

pub fn menu_keyboard() -> ReplyMarkup {
    create_two_button_keyboard("Спасибо!", "Ладно, мне повезет!")
}
