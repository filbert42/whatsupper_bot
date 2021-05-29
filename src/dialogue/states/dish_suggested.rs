use crate::dialogue::{dishes::Dish, Dialogue};
use crate::utils::*;
use teloxide::{prelude::*, types::ReplyMarkup};

#[derive(Clone, Generic)]
struct SuggestedDish {
    variants: Vec<Dish>,
    dish: Dish,
}

impl SuggestedDish {
    fn new(variants: Vec<Dish>, dish: Dish) -> Self {
        SuggestedDish {
            variants: variants,
            dish: dish,
        }
    }
}

#[derive(Clone, Generic)]
pub struct DishSuggestedState {
    suggested: SuggestedDish,
}

#[teloxide(subtransition)]
async fn dish_suggested(
    state: DishSuggestedState,
    cx: TransitionIn<AutoSend<Bot>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    match ans.as_str() {
        "Cпасибо!" => {
            cx.answer(format!("Всегда пожалуйста!"))
                .reply_markup(ReplyMarkup::kb_remove())
                .await?;
            exit()
        }
        "А можно чего другого?" => {
            let rest_variants: Vec<Dish> = state
                .suggested
                .variants
                .iter()
                .filter(|d| d.name != state.suggested.dish.name)
                .collect();
            let chosen_dish = choose_random_food(rest_variants);
            cx.answer("Ой, всё!".to_string()).await?;
            next(DishSuggestedState::up(
                state,
                SuggestedDish::new(rest_variants, chosen_dish),
            ))
        }
        _ => {
            cx.answer("Прости, ничем не могу с этим помочь".to_string())
                .reply_markup(ReplyMarkup::kb_remove())
                .await?;
            exit()
        }
    }
}
