use crate::dialogue::dishes::Dish;
use rand::seq::SliceRandom;

pub fn choose_random_dish(variants: &[Dish]) -> Option<&Dish> {
    variants.choose(&mut rand::thread_rng())
}

pub fn get_dish_variants() -> Vec<Dish> {
    // set executable path as current working directory
    let current_dir = std::env::current_exe()
        .unwrap()
        .as_path()
        .parent()
        .unwrap()
        .to_owned();
    std::env::set_current_dir(&current_dir).unwrap();

    let raw_json = std::fs::read_to_string("./data/dishes_list.json")
        .expect("Something went wrong reading the file");
    let list: Vec<Dish> =
        serde_json::from_str(&raw_json).unwrap_or_else(|_| vec![Dish::new("", "", vec![""])]);
    list
}
