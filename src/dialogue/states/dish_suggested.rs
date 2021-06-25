use crate::dialogue::{dishes::Dish, Dialogue};
use crate::replics;
use crate::utils::*;
use teloxide::prelude::*;

impl DishSuggestedState {
    pub fn new(variants: Vec<Dish>, dish: Dish) -> Self {
        DishSuggestedState { variants, dish }
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
        "Cпасибо!" => replics::thanks_reply(cx, "Всегда пожалуйста!").await,
        "А можно чего другого?" => {
            let rest_variants: Vec<Dish> = state
                .variants
                .iter()
                .filter(|d| d.name != state.dish.name)
                .cloned()
                .collect();
            let chosen_dish = choose_random_dish(&rest_variants);
            replics::chosen_dish_answer(cx, chosen_dish, &rest_variants).await
        }
        _ => replics::sorry_not_sorry(cx).await,
    }
}
