use crate::dialogue::Dialogue;
use teloxide::{prelude::*, types::ReplyMarkup};

use super::ReceiveRequestState;

#[derive(Clone, Generic)]
pub struct DishSuggestedState;

#[teloxide(subtransition)]
async fn dish_suggested(
    _state: DishSuggestedState,
    cx: TransitionIn<AutoSend<Bot>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    match ans.as_str() {
        "Cпасибо!" => {
            cx.answer(format!("Всегда пожалуйста!"))
                .reply_markup(ReplyMarkup::kb_remove())
                .await?;
            exit()
        }
        "А можно чего другого?" => {
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
