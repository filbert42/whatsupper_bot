use crate::dialogue::{dishes::Dish, Dialogue};
use crate::utils::*;
use teloxide::{prelude::*, types::ReplyMarkup};

impl DishSuggestedState {
    pub fn new(variants: Vec<Dish>, dish: Dish) -> Self {
        DishSuggestedState {
            variants: variants,
            dish: dish,
        }
    }
}

#[derive(Clone, Generic)]
pub struct DishSuggestedState {
    variants: Vec<Dish>,
    dish: Dish,
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
                .variants
                .iter()
                .filter(|d| d.name != state.dish.name)
                .cloned()
                .collect();
            let chosen_dish = choose_random_food(&rest_variants);
            match chosen_dish {
                Some(dish) => {
                    cx.answer(dish_suggestion_text(dish))
                        .reply_markup(dish_keyboard())
                        .await?;
                    next(DishSuggestedState::new(rest_variants.clone(), dish.clone()))
                }
                None => {
                    cx.answer("Ой, а блюда-то закончились!".to_string())
                        .reply_markup(ReplyMarkup::kb_remove())
                        .await?;
                    exit()
                }
            }
        }
        _ => {
            cx.answer("Прости, ничем не могу с этим помочь".to_string())
                .reply_markup(ReplyMarkup::kb_remove())
                .await?;
            exit()
        }
    }
}
