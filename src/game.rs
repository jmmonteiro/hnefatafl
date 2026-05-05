use std::collections::HashSet;

use crate::board::{Board, Piece, SpecialSquare, Team};
use crate::cons::NUM_TILES;
use macroquad::prelude::*;

use crate::utils::xy2rowcol;

#[derive(PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    GameOver,
}

pub fn game_loop(
    board: &mut Board,
    legal_moves: &mut HashSet<(usize, usize)>,
    player_turn: &mut Team,
) -> (GameState, String) {
    clear_background(BLACK);

    // Draw board
    board.draw(legal_moves);

    // Check that player has pieces that they can move
    if !board.has_possible_moves(player_turn) {
        return (
            GameState::GameOver,
            match player_turn {
                Team::Attacker => "Attackers have no possible moves. Defenders win.".to_string(),
                Team::Defender => "Defenders have no possible moves. Attackers win.".to_string(),
            },
        );
    }

    // Check that the defenders are not surrounded
    if *player_turn == Team::Defender && board.is_surrounded() {
        return (
            GameState::GameOver,
            "Defenders surrounded by a single circle. Attackers win.".to_string(),
        );
    }

    // Get input
    if is_mouse_button_pressed(MouseButton::Left) {
        let (x, y) = mouse_position();
        let (row, col) = xy2rowcol(x, y);
        match board.selected_square {
            None => {
                if let Some(p) = board.state[row][col]
                    && *player_turn == p.get_team()
                {
                    board.selected_square = Some((row, col));
                    *legal_moves = p.get_legal_moves(row, col, board);
                    return (GameState::Playing, "".to_string());
                }
            }
            Some((player_row, player_col)) => {
                // Move piece
                if legal_moves.contains(&(row, col)) {
                    board.state[row][col] = board.state[player_row][player_col];
                    board.state[player_row][player_col] = None;
                    board.selected_square = None;

                    // Check if the king is in a special square
                    if let (Some(Piece::King(_)), Some(SpecialSquare::Escape)) =
                        (&board.state[row][col], &board.board[row][col])
                    {
                        return (
                            GameState::GameOver,
                            "The king has escaped. Defenders win!".to_string(),
                        );
                    } else {
                        // Check for captures
                        for (r, c) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                            let new_row = (row as i32) + r;
                            let new_col = (col as i32) + c;
                            if !(new_row >= 0
                                && new_col >= 0
                                && new_row < (NUM_TILES as i32)
                                && new_col < (NUM_TILES as i32))
                            {
                                continue;
                            }
                            if let Some(p) = board.state[new_row as usize][new_col as usize] {
                                if p.is_captured(new_row, new_col, board) == GameState::GameOver {
                                    return (
                                        GameState::GameOver,
                                        "The king has been captured. Attackers win!".to_string(),
                                    );
                                }
                            }
                        }
                    };

                    *player_turn = match player_turn {
                        Team::Attacker => Team::Defender,
                        Team::Defender => Team::Attacker,
                    };
                    legal_moves.clear();
                } else if (row, col) == (player_row, player_col) {
                    // Unselect current square
                    board.selected_square = None;
                    legal_moves.clear();
                } else {
                    // Select new square
                    if let Some(p) = board.state[row][col] {
                        board.selected_square = Some((row, col));
                        *legal_moves = p.get_legal_moves(row, col, board);
                    }
                }
            }
        }
    }
    (GameState::Playing, "".to_string())
}
