use crate::dialogue::IngredientChoosingState;
use crate::dialogue::{transitions, Dialogue};
use crate::keyboards::*;
use crate::utils::*;
use teloxide::prelude::*;

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
            let variants = get_dish_variants();
            let chosen_food = choose_random_dish(&variants);
            transitions::chosen_dish_answer(cx, chosen_food, &variants).await
        }
        "А посоветуй-ка мне что-нибудь с..." => {
            cx.answer("Легко! Что бы ты хотел видеть в своем блюде?".to_string())
                .reply_markup(create_keyboard(vec!["Я передумал, назад!"]))
                .await?;
            next(IngredientChoosingState)
        }
        "Огласите весь список!" => transitions::show_full_list(cx).await,
        _ => transitions::sorry_not_sorry(cx).await,
    }
}
