use crate::dialogue::Dialogue;
use teloxide::prelude::*;

#[derive(Clone, Generic)]
pub struct ReceiveRequest {
    pub request_message: String,
}

#[teloxide(subtransition)]
async fn receive_location(
    state: ReceiveLocationState,
    cx: TransitionIn<AutoSend<Bot>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    cx.answer(format!("Твой запрос был: {0}", state.reuest_message))
        .await?;
    exit()
}
