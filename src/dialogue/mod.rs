pub mod dishes;
pub mod replics;
mod states;

pub use crate::dialogue::states::{
    DishSuggestedState, IngredientChoosingState, MenuShowedState, ReceiveRequestState, StartState,
};
use derive_more::From;
use teloxide::macros::Transition;

#[derive(Transition, Clone, From)]
pub enum Dialogue {
    Start(StartState),
    ReceiveRequest(ReceiveRequestState),
    DishSuggested(DishSuggestedState),
    MenuShowed(MenuShowedState),
    IngredientChoosing(IngredientChoosingState),
}

impl Default for Dialogue {
    fn default() -> Self {
        Self::Start(StartState)
    }
}
