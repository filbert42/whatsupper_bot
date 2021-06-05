use crate::{
    dialogue::{states::ReceiveRequestState, Dialogue},
    utils::create_keyboard,
};
use teloxide::{prelude::*, types::ReplyMarkup};

#[derive(Clone)]
pub struct StartState;

#[teloxide(subtransition)]
async fn start(
    _state: StartState,
    cx: TransitionIn<AutoSend<Bot>>,
    _ans: String,
) -> TransitionOut<Dialogue> {
    cx.answer("Привет! Я твой кулинарный помощник! Чем могу помочь?")
        .reply_markup(start_keyboard())
        .await?;
    next(ReceiveRequestState)
}

fn start_keyboard() -> ReplyMarkup {
    create_keyboard(vec![
        "Чего бы мне поесть сегодня?",
        "А посоветуй-ка мне что-нибудь с...",
        "Огласите весь список!",
    ])
}
