use crate::dialogue::dishes::Dish;
use crate::dialogue::states::DishSuggestedState;
use crate::dialogue::Dialogue;
use crate::utils::*;
use teloxide::{prelude::*, types::ReplyMarkup};

#[derive(Clone, Generic)]
pub struct IngredientChoosingState;

#[teloxide(subtransition)]
async fn ingredient_choosing(
    _state: IngredientChoosingState,
    cx: TransitionIn<AutoSend<Bot>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    let chosen_ingredients: Vec<String> = ans
        .as_str()
        .split(',')
        .map(|s| s.trim().to_lowercase())
        .collect();
    let variants: Vec<Dish> = get_food_variants()
        .iter()
        .filter(|d| {
            chosen_ingredients
                .iter()
                .all(|i| d.composition.contains(&i))
        })
        .cloned()
        .collect();
    let chosen_food = choose_random_food(&variants);
    match chosen_food {
        Some(dish) => {
            cx.answer(dish_suggestion_text(dish))
                .reply_markup(dish_keyboard())
                .await?;
            next(DishSuggestedState::new(variants.clone(), dish.clone()))
        }
        None => {
            cx.answer("Ой, а блюда-то закончились!".to_string())
                .reply_markup(ReplyMarkup::kb_remove())
                .await?;
            exit()
        }
    }
}
