use crate::dialogue::{replics, Dialogue};
use crate::utils::*;
use teloxide::prelude::*;

#[derive(Clone, Generic)]
pub struct MenuShowedState;

#[teloxide(subtransition)]
async fn menu_showed(
    _state: MenuShowedState,
    cx: TransitionIn<AutoSend<Bot>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    match ans.as_str() {
        "Спасибо!" => replics::thanks_reply(cx, "Пожалуйста!").await,
        "Ладно, мне повезет!" => {
            let variants = get_dish_variants();
            let chosen_food = choose_random_dish(&variants);
            replics::chosen_dish_answer(cx, chosen_food, &variants).await
        }
        _ => replics::sorry_not_sorry(cx).await,
    }
}
