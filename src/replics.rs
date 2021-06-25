use crate::dialogue::dishes::Dish;
use crate::dialogue::{Dialogue, DishSuggestedState, MenuShowedState};
use crate::keyboards::*;
use crate::utils::*;
use teloxide::prelude::*;
use teloxide::types::ReplyMarkup;

pub fn dish_suggestion_text(dish: &Dish) -> String {
    format!(
        "Я предлагаю тебе отведать сегодня:\n{0}",
        dish.format_to_string()
    )
}
pub async fn no_more_dishes(cx: TransitionIn<AutoSend<Bot>>) -> TransitionOut<Dialogue> {
    cx.answer("Ой, а блюда-то закончились!".to_string())
        .reply_markup(ReplyMarkup::kb_remove())
        .await?;
    exit()
}

pub async fn sorry_not_sorry(cx: TransitionIn<AutoSend<Bot>>) -> TransitionOut<Dialogue> {
    cx.answer("Прости, ничем не могу с этим помочь".to_string())
        .reply_markup(ReplyMarkup::kb_remove())
        .await?;
    exit()
}

pub async fn chosen_dish_answer(
    cx: TransitionIn<AutoSend<Bot>>,
    chosen_dish: Option<&Dish>,
    rest_variants: &Vec<Dish>,
) -> TransitionOut<Dialogue> {
    match chosen_dish {
        Some(dish) => {
            cx.answer(dish_suggestion_text(dish))
                .reply_markup(dish_keyboard())
                .await?;
            next(DishSuggestedState::new(rest_variants.clone(), dish.clone()))
        }
        None => no_more_dishes(cx).await,
    }
}

pub async fn show_full_list(cx: TransitionIn<AutoSend<Bot>>) -> TransitionOut<Dialogue> {
    let full_list = get_dish_variants()
        .iter()
        .map(|d| d.format_to_string())
        .collect::<Vec<String>>()
        .join("\n\n");
    cx.answer(full_list).reply_markup(menu_keyboard()).await?;
    next(MenuShowedState)
}

pub async fn thanks_reply(
    cx: TransitionIn<AutoSend<Bot>>,
    wording: &str,
) -> TransitionOut<Dialogue> {
    cx.answer(wording.to_string())
        .reply_markup(ReplyMarkup::kb_remove())
        .await?;
    exit()
}
