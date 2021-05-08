use crate::dialogue::{states::ReceiveRequest, Dialogue};
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
        .await?;
    next(ReceiveRequest)
}
