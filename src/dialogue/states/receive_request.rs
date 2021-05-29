use crate::dialogue::{
    states::dish_suggested::DishSuggestedState, states::menu_showed::MenuShowedState, Dialogue,
};
use crate::utils::*;
use teloxide::{prelude::*, types::ReplyMarkup};

#[derive(Clone, Generic)]
pub struct ReceiveRequestState;

#[teloxide(subtransition)]
async fn receive_request(
    _state: ReceiveRequestState,
    cx: TransitionIn<AutoSend<Bot>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    match ans.as_str() {
        "Чего бы мне поесть сегодня?" => {
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
        "Огласите весь список!" => {
            let full_list = get_food_variants()
                .iter()
                .map(|d| d.format_to_string())
                .collect::<Vec<String>>()
                .join("\n\n");
            cx.answer(full_list).reply_markup(menu_keyboard()).await?;
            next(MenuShowedState)
        }
        _ => {
            cx.answer("Прости, ничем не могу с этим помочь".to_string())
                .reply_markup(ReplyMarkup::kb_remove())
                .await?;
            exit()
        }
    }
}
