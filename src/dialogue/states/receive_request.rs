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
            let chosen_food = choose_random_food(variants);
            cx.answer(chosen_food.format_to_string())
                .reply_markup(dish_keyboard())
                .await?;
            next(DishSuggestedState)
        }
        "Огласите весь список!" => {
            cx.answer("Ой, всё!".to_string())
                .reply_markup(menu_keyboard())
                .await?;
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

fn dish_keyboard() -> ReplyMarkup {
    create_two_button_keyboard("Cпасибо!", "А можно чего другого?")
}

fn menu_keyboard() -> ReplyMarkup {
    create_two_button_keyboard("Спасибо!", "Ладно, мне повезет!")
}
