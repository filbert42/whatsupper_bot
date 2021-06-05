use super::DishSuggestedState;
use crate::dialogue::Dialogue;
use crate::utils::*;
use teloxide::{prelude::*, types::ReplyMarkup};

#[derive(Clone, Generic)]
pub struct MenuShowedState;

#[teloxide(subtransition)]
async fn menu_showed(
    _state: MenuShowedState,
    cx: TransitionIn<AutoSend<Bot>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    match ans.as_str() {
        "Спасибо!" => {
            cx.answer("Пожалуйста!".to_string())
                .reply_markup(ReplyMarkup::kb_remove())
                .await?;
            exit()
        }
        "Ладно, мне повезет!" => {
            let variants = get_food_variants();
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
        _ => {
            cx.answer("Прости, ничем не могу с этим помочь".to_string())
                .reply_markup(ReplyMarkup::kb_remove())
                .await?;
            exit()
        }
    }
}
