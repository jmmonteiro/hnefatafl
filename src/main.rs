use std::collections::HashSet;

use macroquad::conf::UpdateTrigger;
use macroquad::prelude::*;

pub const TILE_SIZE: f32 = 50.;

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

fn rowcol2xy(row: usize, col: usize) -> (f32, f32) {
    (
        (col as f32) * (TILE_SIZE + 1.),
        (row as f32) * (TILE_SIZE + 1.),
    )
}

// TODO: The math is not 100% correct, does not account for the border between each tile
// but in practice the difference should be small. Fix later if I can be arsed
fn xy2rowcol(x: f32, y: f32) -> (usize, usize) {
    ((y / TILE_SIZE) as usize, (x / TILE_SIZE) as usize)
}

fn draw_piece(x: f32, y: f32, color: Color) {
    draw_circle(
        x + (TILE_SIZE + 1.) / 2.0,
        y + (TILE_SIZE + 1.) / 2.0,
        TILE_SIZE / 3.,
        color,
    );
}

fn draw_board(board_state: &[[u8; 11]; 11], legal_moves: &HashSet<(usize, usize)>) {
    (0..11).for_each(|r| {
        (0..11).for_each(|c| {
            let (x, y) = rowcol2xy(r, c);
            draw_rectangle(
                x,
                y,
                TILE_SIZE,
                TILE_SIZE,
                if (r == 0 && (c == 0 || c == 10))
                    || (c == 0 && (r == 0 || r == 10))
                    || (r == 10 && c == 10)
                {
                    GRAY
                } else {
                    BEIGE
                },
            );
            if board_state[r][c] >= 10 {
                draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, GOLD);
            }
            for (r, c) in legal_moves {
                let (x, y) = rowcol2xy(*r, *c);
                draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, LIME);
            }
            match board_state[r][c] {
                1 | 10 => {
                    draw_piece(x, y, RED);
                }
                2 | 20 => {
                    draw_piece(x, y, WHITE);
                }
                3 | 30 => {
                    draw_piece(x, y, PURPLE);
                }
                _ => {}
            }
        });
    });
}

fn add_legal_moves(
    row: usize,
    col: usize,
    board_state: &[[u8; 11]; 11],
    legal_moves: &mut HashSet<(usize, usize)>,
) {
    for i in (0..row).rev().take_while(|&i| board_state[i][col] == 0) {
        legal_moves.insert((i, col));
    }
    for i in (row + 1..11).take_while(|&i| board_state[i][col] == 0) {
        legal_moves.insert((i, col));
    }
    for i in (0..col).rev().take_while(|&i| board_state[row][i] == 0) {
        legal_moves.insert((row, i));
    }
    for i in (col + 1..11).take_while(|&i| board_state[row][i] == 0) {
        legal_moves.insert((row, i));
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // States:
    // empty: 0
    // attacker: 1
    // defender: 2
    // king: 3
    let mut board_state: [[u8; 11]; 11] = [
        [0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 2, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 2, 2, 2, 0, 0, 0, 1],
        [1, 1, 0, 2, 2, 3, 2, 2, 0, 1, 1],
        [1, 0, 0, 0, 2, 2, 2, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 2, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0],
    ];

    let mut player_turn: u8 = 1;
    let mut tile_selected: Option<(usize, usize)> = None;
    let mut legal_moves: HashSet<(usize, usize)> = HashSet::new();

    loop {
        clear_background(BLACK);

        // Draw board
        draw_board(&board_state, &legal_moves);

        // Get input
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            let (row, col) = xy2rowcol(x, y);
            match tile_selected {
                None => {
                    if (player_turn == 1 && board_state[row][col] == 1)
                        || (player_turn == 2
                            && (board_state[row][col] == 2 || board_state[row][col] == 3))
                    {
                        board_state[row][col] *= 10;
                        // add legal moves to the set
                        tile_selected = Some((row, col));
                        add_legal_moves(row, col, &board_state, &mut legal_moves);
                    }
                }
                Some((player_row, player_col)) => {
                    if legal_moves.contains(&(row, col)) {
                        //  Move piece
                        board_state[row][col] = board_state[player_row][player_col] / 10;
                        board_state[player_row][player_col] = 0;
                        tile_selected = None;
                        legal_moves.clear();
                        if player_turn == 1 {
                            player_turn = 2;
                        } else {
                            player_turn = 1;
                        }
                    } else if (row, col) == (player_row, player_col) {
                        board_state[row][col] /= 10;
                        tile_selected = None;
                        legal_moves.clear();
                    }
                }
            }
        }

        next_frame().await
    }
}
