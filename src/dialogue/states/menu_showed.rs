use crate::dialogue::Dialogue;
use teloxide::{prelude::*, types::ReplyMarkup};

use super::ReceiveRequestState;

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
            cx.answer(format!("Пожалуйста!"))
                .reply_markup(ReplyMarkup::kb_remove())
                .await?;
            exit()
        }
        "Ладно, мне повезет!" => {
            cx.answer("Ой, всё!".to_string()).await?;
            next(ReceiveRequestState)
        }
        _ => {
            cx.answer("Прости, ничем не могу с этим помочь".to_string())
                .reply_markup(ReplyMarkup::kb_remove())
                .await?;
            exit()
        }
    }
}
