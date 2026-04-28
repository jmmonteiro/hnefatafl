use macroquad::prelude::*;
use std::collections::HashSet;

use crate::cons::TILE_SIZE;

pub enum SpecialSquare {
    Escape,
    Throne,
}

#[derive(Copy, Clone)]
pub enum Team {
    Attacker,
    Defender,
}

#[derive(Copy, Clone)]
pub struct Soldier {
    pub team: Team,
}
impl Soldier {
    fn new(team: Team) -> Soldier {
        Soldier { team }
    }
}

#[derive(Copy, Clone)]
pub struct King {
    pub team: Team,
}

impl King {
    fn new() -> King {
        King {
            team: Team::Defender,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Piece {
    Soldier(Soldier),
    King(King),
}

pub struct Board {
    pub board: Vec<Vec<Option<SpecialSquare>>>,
    pub state: Vec<Vec<Option<Piece>>>,
    pub selected_square: Option<(usize, usize)>,
}

impl Board {
    pub fn new() -> Board {
        let mut board: Vec<Vec<Option<SpecialSquare>>> = (0..11)
            .map(|_| -> Vec<Option<SpecialSquare>> { (0..11).map(|_| None).collect() })
            .collect();

        for (r, c) in [(0, 0), (0, 10), (10, 0), (10, 10)] {
            board[r][c] = Some(SpecialSquare::Escape);
        }
        board[5][5] = Some(SpecialSquare::Throne);

        let state_int: [[u8; 11]; 11] = [
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
        let state: Vec<Vec<Option<Piece>>> = (0..11)
            .map(|r| {
                (0..11)
                    .map(|c| match state_int[r][c] {
                        1 => Some(Piece::Soldier(Soldier::new(Team::Attacker))),
                        2 => Some(Piece::Soldier(Soldier::new(Team::Defender))),
                        3 => Some(Piece::King(King::new())),
                        _ => None,
                    })
                    .collect()
            })
            .collect();

        Board {
            board,
            state,
            selected_square: None,
        }
    }

    pub fn draw(&self, legal_moves: &HashSet<(usize, usize)>) {
        // TODO: The math is not 100% correct, does not account for the border between each tile
        // but in practice the difference should be small. Fix later if I can be arsed

        fn rowcol2xy(row: usize, col: usize) -> (f32, f32) {
            (
                (col as f32) * (TILE_SIZE + 1.),
                (row as f32) * (TILE_SIZE + 1.),
            )
        }
        fn draw_piece(x: f32, y: f32, color: Color) {
            draw_circle(
                x + (TILE_SIZE + 1.) / 2.0,
                y + (TILE_SIZE + 1.) / 2.0,
                TILE_SIZE / 3.,
                color,
            );
        }

        (0..11).for_each(|r| {
            (0..11).for_each(|c| {
                let (x, y) = rowcol2xy(r, c);
                draw_rectangle(
                    x,
                    y,
                    TILE_SIZE,
                    TILE_SIZE,
                    if self.board[r][c].is_some() {
                        GRAY
                    } else {
                        BEIGE
                    },
                );
                match self.selected_square {
                    Some(s) if s == (r, c) => draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, GOLD),
                    _ => {}
                }
                for (r, c) in legal_moves {
                    let (x, y) = rowcol2xy(*r, *c);
                    draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, LIME);
                }
                match &self.state[r][c] {
                    None => {}
                    Some(Piece::King(_)) => {
                        draw_piece(x, y, PURPLE);
                    }
                    Some(Piece::Soldier(s)) => match s.team {
                        Team::Attacker => {
                            draw_piece(x, y, RED);
                        }
                        Team::Defender => {
                            draw_piece(x, y, WHITE);
                        }
                    },
                }
            });
        });
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
