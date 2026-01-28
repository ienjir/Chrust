use chrust_core_simple::{Square, moves::move_gen::move_gen::get_possible_moves};

use crate::{state::{GameState}};

pub enum UiEvent {
    ClickSquare(Square),
    ClickResetButton, 
}

pub fn apply_ui_event(game_state: &mut GameState, ui_event: UiEvent) {
    match ui_event {
        UiEvent::ClickSquare(square) => {
            click_square(game_state, square);
        }
        UiEvent::ClickResetButton => {
            println!("Reset board (currently not implemented)");
        }
    }
}

pub fn click_square(game_state: &mut GameState, from_square: Square) {
    if game_state.selected.is_none() {
        let square_occupant = match game_state.position.board[from_square as usize] {
            Some(p) => p,
            None => {
                game_state.possible_moves.clear();
                return;
            }
        };

        if square_occupant.side != game_state.position.side_to_move {
            game_state.possible_moves.clear();
            return;
        }

        match get_possible_moves(&game_state.position, from_square) {
            Ok(moves) => {
                game_state.selected = Some(from_square);
                game_state.possible_moves = moves;
                return;
            }
            Err(x) => {
                println!("Error occured, please implement error handling");
                game_state.selected = None;
                game_state.possible_moves.clear();
                return;
            }
        }
    } else {
        let Some(selected_square) = game_state.selected else { return; };
        let clicked_occupant = game_state.position.board[from_square as usize];

        if from_square == selected_square {
            game_state.possible_moves.clear();
            game_state.selected = None;
            return;
        }

        if let Some(chosen_move) = game_state.possible_moves.iter().find(|m| m.to_square == from_square) {
            match game_state.position.make_move_unvalidated(chosen_move.from_square, chosen_move.to_square) {
                    Ok(p) => {
                        game_state.position = p;
                        game_state.possible_moves.clear();
                        game_state.selected = None;
                        return;
                    }
                    Err(_e) => {
                        println!("Error occured, please implement error handling");
                        game_state.selected = None;
                        game_state.possible_moves.clear();
                        return;
                    }
                }
        }

        if let Some(piece) = clicked_occupant {
            if piece.side == game_state.position.side_to_move {
                match get_possible_moves(&game_state.position, from_square) {
                    Ok(moves) => {
                        game_state.selected = Some(from_square);
                        game_state.possible_moves = moves;
                        return;
                    }
                    Err(x) => {
                        println!("Error occured, please implement error handling");
                        game_state.selected = None;
                        game_state.possible_moves.clear();
                        return;
                    }
                }
            }
        }

        game_state.possible_moves.clear();
        game_state.selected = None;
    }
} 
