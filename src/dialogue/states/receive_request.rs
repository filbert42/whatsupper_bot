use crate::dialogue::dishes::Dish;
use crate::dialogue::Dialogue;
use rand::seq::SliceRandom;
use teloxide::prelude::*;

#[derive(Clone, Generic)]
pub struct ReceiveRequestState;

#[teloxide(subtransition)]
async fn receive_request(
    _state: ReceiveRequestState,
    cx: TransitionIn<AutoSend<Bot>>,
    _ans: String,
) -> TransitionOut<Dialogue> {
    let variants = get_food_variants();
    let chosen_food = choose_random_food(variants);
    cx.answer(chosen_food.format_to_string()).await?;
    exit()
}

fn choose_random_food(variants: Vec<Dish>) -> Dish {
    variants.choose(&mut rand::thread_rng()).unwrap().clone()
}

fn get_food_variants() -> Vec<Dish> {
    vec![
        Dish::new(
            "Хумус".to_string(),
            "Вкуснейшая ближневосточная закуска.".to_string(),
            vec!["нут", "тхина", "оливковое масло", "чеснок", "лимонный сок"]
                .iter()
                .map(|&s| s.to_string())
                .collect::<Vec<String>>(),
        ),
        Dish::new(
            "Маджадра".to_string(),
            "Главное вегетарианское блюдо на свете".to_string(),
            vec!["рис", "чечевица", "лук"]
                .iter()
                .map(|&s| s.to_string())
                .collect::<Vec<String>>(),
        ),
        Dish::new(
            "Шахи Панир".to_string(),
            "Нежный индийский сыр в томатном соусе".to_string(),
            vec!["панир", "помидоры", "перец"]
                .iter()
                .map(|&s| s.to_string())
                .collect::<Vec<String>>(),
        ),
    ]
}
