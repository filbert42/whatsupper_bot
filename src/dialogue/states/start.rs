use crate::{
    dialogue::{states::ReceiveRequestState, Dialogue},
    utils::*,
};
use teloxide::prelude::*;

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
