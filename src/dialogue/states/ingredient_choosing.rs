use crate::dialogue::dishes::Dish;
use crate::dialogue::{replics, Dialogue};
use crate::keyboards::*;
use crate::utils::*;
use teloxide::prelude::*;

use super::ReceiveRequestState;

#[derive(Clone, Generic)]
pub struct IngredientChoosingState;

#[teloxide(subtransition)]
async fn ingredient_choosing(
    _state: IngredientChoosingState,
    cx: TransitionIn<AutoSend<Bot>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    match ans.as_str() {
        "Я передумал, назад!" => {
            cx.answer("Нет проблем!".to_string())
                .reply_markup(start_keyboard())
                .await?;
            next(ReceiveRequestState)
        }
        _ => {
            let chosen_ingredients: Vec<String> = ans
                .as_str()
                .split(',')
                .map(|s| s.trim().to_lowercase())
                .collect();
            let variants: Vec<Dish> = get_dish_variants()
                .iter()
                .filter(|d| {
                    chosen_ingredients
                        .iter()
                        .all(|i| d.composition.contains(&i))
                })
                .cloned()
                .collect();
            let chosen_food = choose_random_dish(&variants);
            replics::chosen_dish_answer(cx, chosen_food, &variants).await
        }
    }
}
