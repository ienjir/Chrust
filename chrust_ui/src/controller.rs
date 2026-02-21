use chrust_core::{Piece, Square, moves::{make_move::{MoveKind}, move_gen::move_gen::get_possible_moves}};

use crate::state::{GameState, UiState};

pub enum UiEvent {
    ClickSquare(Square),
    ClickPromotionSquare(Piece),
    ClickResetButton, 
}

pub fn apply_ui_event(game_state: &mut GameState, ui_event: UiEvent) {
    match ui_event {
        UiEvent::ClickResetButton => {
            println!("Reset board (currently not implemented)");
        }
        UiEvent::ClickPromotionSquare(piece) => {
            click_promotion(game_state, piece); 
        }
        UiEvent::ClickSquare(square) => {
            click_square(game_state, square);
        }
    }
}

pub fn click_promotion(game_state: &mut GameState, piece: Piece) {
    if piece == Piece::Pawn {
        println!("Promotion piece is pawn");
        return;
    }

    let state = game_state.ui_state.take();

    let mut mv = match state {
        Some(UiState::PROMOTION { pending_move, .. }) => pending_move,
        _ => {
            println!("Gamestate is not promotion");
            return;
        }
    };

    mv.move_kind = MoveKind::Promotion { promotion_piece: Some(piece) };

    match game_state.position.make_move_validated(&mv) {
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
    };
}

pub fn click_square(game_state: &mut GameState, from_square: Square) {
    if matches!(game_state.ui_state, Some(UiState::PROMOTION { .. })) {
        return;
    }

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

        // Make move 
        if let Some(chosen_move) = game_state.possible_moves.iter().find(|m| m.to_square == from_square) {
            if matches!(chosen_move.move_kind, MoveKind::Promotion { .. }) {
                game_state.ui_state = Some(UiState::PROMOTION { pending_move: chosen_move.clone()});
                game_state.selected = None;
                game_state.possible_moves.clear();
                return;
            }

            match game_state.position.make_move_validated(&chosen_move) {
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
