use chrust_core_simple::Square;
use crate::{state::{GameState}};

pub enum UiEvent {
    ClickSquare(Square),
    ClickResetButton, 
}

pub fn apply_ui_event(game_state: &mut GameState, ui_event: UiEvent) {
    match ui_event {
        UiEvent::ClickSquare(square) => {
            game_state.highlighted = Some(square);
        }
        UiEvent::ClickResetButton => {
            println!("Reset board (currently not implemented)");
        }
    }
}

