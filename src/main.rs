//  Fetlar Hnefatafl
// https://aagenielsen.dk/fetlar_rules_en.php

use std::collections::HashSet;

use hnefatafl::board::{Board, GameState, Piece, SpecialSquare, Team};
use hnefatafl::cons::{NUM_TILES, TILE_SIZE};
use hnefatafl::utils::xy2rowcol;
use macroquad::conf::UpdateTrigger;
use macroquad::prelude::*;

fn window_conf() -> macroquad::conf::Conf {
    let window_size = (TILE_SIZE as i32) * (NUM_TILES as i32) + (NUM_TILES as i32);
    macroquad::conf::Conf {
        miniquad_conf: Conf {
            window_title: "Hnefatafl".to_owned(),
            fullscreen: false,
            window_width: window_size,
            window_height: window_size,
            window_resizable: false,
            ..Default::default()
        },
        update_on: Some(UpdateTrigger {
            key_down: true,
            mouse_down: true,
            mouse_motion: false,
            ..Default::default()
        }),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player_turn = Team::Attacker;
    let mut legal_moves: HashSet<(usize, usize)> = HashSet::new();

    let mut board = Board::default();
    let mut game_state = GameState::Playing;

    loop {
        clear_background(BLACK);

        // Draw board
        board.draw(&legal_moves);

        // Check that player has pieces that they can move
        if !board.has_possible_moves(&player_turn) {
            match player_turn {
                Team::Attacker => {
                    game_state = GameState::DefenderWins;
                    println!("Attackers have no possible moves. Defenders win.")
                }
                Team::Defender => {
                    game_state = GameState::AttackerWins;
                    println!("Defenders have no possible moves. Attackers win.")
                }
            }
        }

        // Get input
        if game_state == GameState::Playing && is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            let (row, col) = xy2rowcol(x, y);
            match board.selected_square {
                None => {
                    if let Some(p) = board.state[row][col]
                        && player_turn == p.get_team()
                    {
                        board.selected_square = Some((row, col));
                        legal_moves = p.get_legal_moves(row, col, &board);
                    }
                }
                Some((player_row, player_col)) => {
                    // Move piece
                    if legal_moves.contains(&(row, col)) {
                        board.state[row][col] = board.state[player_row][player_col];
                        board.state[player_row][player_col] = None;
                        board.selected_square = None;

                        // TODO: Check that the defenders are not surrounded
                        // https://aagenielsen.dk/fetlar_rules_en.php

                        // Check if the king is in a special square
                        game_state = if let (Some(Piece::King(_)), Some(SpecialSquare::Escape)) =
                            (&board.state[row][col], &board.board[row][col])
                        {
                            GameState::DefenderWins
                        } else {
                            // Check for captures
                            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                                .iter()
                                .find_map(|(r, c)| {
                                    let new_row = (row as i32) + r;
                                    let new_col = (col as i32) + c;
                                    if new_row >= 0
                                        && new_col >= 0
                                        && new_row < (NUM_TILES as i32)
                                        && new_col < (NUM_TILES as i32)
                                    {
                                        match board.state[new_row as usize][new_col as usize] {
                                            Some(p) => {
                                                match p.is_captured(new_row, new_col, &mut board) {
                                                    GameState::Playing => None,
                                                    state => Some(state),
                                                }
                                            }
                                            None => None,
                                        }
                                    } else {
                                        None
                                    }
                                })
                                .unwrap_or(GameState::Playing)
                        };

                        match game_state {
                            GameState::Playing => {}
                            GameState::AttackerWins => {
                                println!("Attackers Win.")
                            }
                            GameState::DefenderWins => {
                                println!("Defenders Win.")
                            }
                        }
                        player_turn = match player_turn {
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
                            legal_moves = p.get_legal_moves(row, col, &board);
                        }
                    }
                }
            }
        }

        next_frame().await
    }
}
