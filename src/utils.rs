use crate::dialogue::dishes::Dish;
use rand::seq::SliceRandom;
use teloxide::types::{KeyboardButton, KeyboardMarkup, ReplyMarkup};

pub fn create_keyboard(buttons_texts: Vec<&str>) -> ReplyMarkup {
    let buttons: Vec<KeyboardButton> = buttons_texts
        .iter()
        .map(|b| KeyboardButton::new(b.to_string()))
        .collect();
    let keyboard = KeyboardMarkup::new(vec![buttons]).resize_keyboard(true);
    ReplyMarkup::Keyboard(keyboard)
}

pub fn choose_random_food(variants: &[Dish]) -> Option<&Dish> {
    variants.choose(&mut rand::thread_rng())
}

pub fn get_food_variants() -> Vec<Dish> {
    let raw_json = std::fs::read_to_string("./src/data/dishes_list.json")
        .expect("Something went wrong reading the file");
    let list: Vec<Dish> =
        serde_json::from_str(&raw_json).unwrap_or_else(|_| vec![Dish::new("", "", vec![""])]);
    list
}

pub fn dish_suggestion_text(dish: &Dish) -> String {
    format!(
        "Я предлагаю тебе отведать сегодня:\n{0}",
        dish.format_to_string()
    )
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
