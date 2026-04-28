use std::collections::HashSet;

use macroquad::conf::UpdateTrigger;
use macroquad::prelude::*;
use tafl::board::{Board, Piece, Team};
use tafl::cons::TILE_SIZE;
use tafl::utils::xy2rowcol;

fn window_conf() -> macroquad::conf::Conf {
    let window_size = (TILE_SIZE as i32) * 11 + 11;
    macroquad::conf::Conf {
        miniquad_conf: Conf {
            window_title: "Taft".to_owned(),
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

fn add_legal_moves(
    row: usize,
    col: usize,
    board_state: &Vec<Vec<Option<Piece>>>,
    legal_moves: &mut HashSet<(usize, usize)>,
) {
    legal_moves.clear();
    for i in (0..row)
        .rev()
        .take_while(|&i| board_state[i][col].is_none())
    {
        legal_moves.insert((i, col));
    }
    for i in (row + 1..11).take_while(|&i| board_state[i][col].is_none()) {
        legal_moves.insert((i, col));
    }
    for i in (0..col)
        .rev()
        .take_while(|&i| board_state[row][i].is_none())
    {
        legal_moves.insert((row, i));
    }
    for i in (col + 1..11).take_while(|&i| board_state[row][i].is_none()) {
        legal_moves.insert((row, i));
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player_turn = Team::Attacker;
    let mut legal_moves: HashSet<(usize, usize)> = HashSet::new();

    let mut board = Board::new();

    loop {
        clear_background(BLACK);

        // Draw board
        board.draw(&legal_moves);

        // Get input
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            let (row, col) = xy2rowcol(x, y);
            match board.selected_square {
                None => match board.state[row][col] {
                    None => {}
                    Some(Piece::King(_)) => {
                        if let Team::Defender = player_turn {
                            board.selected_square = Some((row, col));
                            add_legal_moves(row, col, &board.state, &mut legal_moves);
                        }
                    }
                    Some(Piece::Soldier(s)) => match (player_turn, s.team) {
                        (Team::Defender, Team::Defender) | (Team::Attacker, Team::Attacker) => {
                            board.selected_square = Some((row, col));
                            add_legal_moves(row, col, &board.state, &mut legal_moves);
                        }
                        _ => {}
                    },
                },
                Some((player_row, player_col)) => {
                    if legal_moves.contains(&(row, col)) {
                        // Move piece
                        board.state[row][col] = board.state[player_row][player_col];
                        board.state[player_row][player_col] = None;
                        board.selected_square = None;
                        player_turn = match player_turn {
                            Team::Attacker => Team::Defender,
                            Team::Defender => Team::Attacker,
                        }
                    } else if (row, col) == (player_row, player_col) {
                        // Unselect current square
                        board.selected_square = None;
                        legal_moves.clear();
                    } else {
                        // Select new square
                        match board.state[row][col] {
                            Some(Piece::King(_)) => {
                                if let Team::Defender = player_turn {
                                    board.selected_square = Some((row, col));
                                    add_legal_moves(row, col, &board.state, &mut legal_moves);
                                }
                            }
                            Some(Piece::Soldier(s)) => match (s.team, player_turn) {
                                (Team::Defender, Team::Defender)
                                | (Team::Attacker, Team::Attacker) => {
                                    board.selected_square = Some((row, col));
                                    add_legal_moves(row, col, &board.state, &mut legal_moves);
                                }
                                _ => {}
                            },
                            None => {}
                        }
                    }
                }
            }
        }

        next_frame().await
    }
}
