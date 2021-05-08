mod dishes;
mod states;

use crate::dialogue::states::{
    DishSuggestedState, MenuShowedState, ReceiveRequestState, StartState,
};
use derive_more::From;
use teloxide::macros::Transition;

#[derive(Transition, Clone, From)]
pub enum Dialogue {
    Start(StartState),
    ReceiveRequest(ReceiveRequestState),
    DishSuggested(DishSuggestedState),
    MenuShowed(MenuShowedState),
}

impl Default for Dialogue {
    fn default() -> Self {
        Self::Start(StartState)
    }
}
