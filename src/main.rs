//  Fetlar Hnefatafl
// https://aagenielsen.dk/fetlar_rules_en.php

use std::collections::HashSet;

use hnefatafl::board::{Board, Team};
use hnefatafl::game::{GameState, game_loop};
use hnefatafl::ui::{game_over, get_menu};
use macroquad::conf::UpdateTrigger;
use macroquad::prelude::*;

fn window_conf() -> macroquad::conf::Conf {
    let window_size = 500;
    macroquad::conf::Conf {
        miniquad_conf: Conf {
            window_title: "Hnefatafl".to_owned(),
            fullscreen: false,
            window_width: window_size,
            window_height: window_size,
            window_resizable: true,
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
    let mut game_state = GameState::Menu;
    let mut message = "".to_string();

    loop {
        match game_state {
            GameState::Menu => {
                board.draw(&legal_moves);
                get_menu(&mut game_state);
            }
            GameState::Playing => match game_loop(&mut board, &mut legal_moves, &mut player_turn) {
                (GameState::Playing, _) => {}
                (GameState::Menu, _) => {}
                (GameState::GameOver, m) => {
                    game_state = GameState::GameOver;
                    message = m;
                }
            },
            GameState::GameOver => {
                legal_moves.clear();
                board.draw(&legal_moves);
                game_over(&mut game_state, &mut board, &message);
            }
        }
        next_frame().await
    }
}
