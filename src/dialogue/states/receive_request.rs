use crate::dialogue::Dialogue;
use teloxide::prelude::*;

#[derive(Clone, Generic)]
pub struct ReceiveRequest;

#[teloxide(subtransition)]
async fn receive_request(
    state: ReceiveRequest,
    cx: TransitionIn<AutoSend<Bot>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    cx.answer(format!("Твой запрос был: {0}", ans)).await?;
    exit()
}
