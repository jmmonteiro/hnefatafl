use macroquad::prelude::*;
use std::collections::HashSet;

use crate::cons::{NUM_TILES, TILE_SIZE};

#[derive(PartialEq)]
pub enum GameState {
    Playing,
    AttackerWins,
    DefenderWins,
}

pub enum SpecialSquare {
    Escape,
    Throne,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Team {
    Attacker,
    Defender,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Soldier {
    pub team: Team,
}
impl Soldier {
    fn new(team: Team) -> Soldier {
        Soldier { team }
    }
}

#[derive(Copy, Clone, PartialEq)]
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

#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
    Soldier(Soldier),
    King(King),
}

pub trait Move {
    fn add_legal_moves(
        &self,
        row: usize,
        col: usize,
        board: &Board,
        legal_moves: &mut HashSet<(usize, usize)>,
    ) {
        legal_moves.clear();
        for i in (0..row)
            .rev()
            .take_while(|&i| board.state[i][col].is_none())
        {
            if self.is_square_allowed(i, col, board) {
                legal_moves.insert((i, col));
            }
        }
        for i in (row + 1..NUM_TILES).take_while(|&i| board.state[i][col].is_none()) {
            if self.is_square_allowed(i, col, board) {
                legal_moves.insert((i, col));
            }
        }
        for i in (0..col)
            .rev()
            .take_while(|&i| board.state[row][i].is_none())
        {
            if self.is_square_allowed(row, i, board) {
                legal_moves.insert((row, i));
            }
        }
        for i in (col + 1..NUM_TILES).take_while(|&i| board.state[row][i].is_none()) {
            if self.is_square_allowed(row, i, board) {
                legal_moves.insert((row, i));
            }
        }
    }
    fn is_square_allowed(&self, row: usize, col: usize, board: &Board) -> bool;
}

impl Move for King {
    fn is_square_allowed(&self, _: usize, _: usize, _: &Board) -> bool {
        true
    }
}
impl Move for Soldier {
    fn is_square_allowed(&self, row: usize, col: usize, board: &Board) -> bool {
        board.board[row][col].is_none()
    }
}

impl Piece {
    fn get_team(&self, piece: &Piece) -> Team {
        match piece {
            Piece::Soldier(p) => p.team,
            Piece::King(p) => p.team,
        }
    }
    fn is_hostile_square(&self, row: i32, col: i32, board: &Board) -> bool {
        let row = row as usize;
        let col = col as usize;

        // Special Squares
        if let Some(s) = &board.board[row][col] {
            // If the piece is a defender and the throne is occupied, then it's not and
            // hostile square
            if matches!(s, SpecialSquare::Throne)
                && board.state[row][col].is_some()
                && self.get_team(self) == Team::Defender
            {
                return false;
            }

            return true;
        }

        // Enemy present
        board.state[row][col]
            .map(|p| self.get_team(&p) != self.get_team(self))
            .unwrap_or(false)
    }

    pub fn is_captured(&self, row: i32, col: i32, board: &mut Board) -> GameState {
        match board.state[row as usize][col as usize] {
            None => GameState::Playing,
            Some(p) => {
                match p {
                    Piece::Soldier(_) => {
                        if (self.is_hostile_square(row - 1, col, board)
                            && self.is_hostile_square(row + 1, col, board))
                            || (self.is_hostile_square(row, col - 1, board)
                                && self.is_hostile_square(row, col + 1, board))
                        {
                            board.state[row as usize][col as usize] = None;
                        }
                        return GameState::Playing;
                    }
                    Piece::King(_) => {
                        if self.is_hostile_square(row - 1, col, board)
                            && self.is_hostile_square(row + 1, col, board)
                            && self.is_hostile_square(row, col - 1, board)
                            && self.is_hostile_square(row, col + 1, board)
                        {
                            return GameState::AttackerWins;
                        }
                    }
                }
                GameState::Playing
            }
        }
    }
}

pub struct Board {
    pub board: Vec<Vec<Option<SpecialSquare>>>,
    pub state: Vec<Vec<Option<Piece>>>,
    pub selected_square: Option<(usize, usize)>,
}

impl Board {
    pub fn new(initial_state: [[u8; NUM_TILES]; NUM_TILES]) -> Board {
        let mut board: Vec<Vec<Option<SpecialSquare>>> = (0..NUM_TILES)
            .map(|_| -> Vec<Option<SpecialSquare>> { (0..NUM_TILES).map(|_| None).collect() })
            .collect();

        for (r, c) in [(0, 0), (0, 10), (10, 0), (10, 10)] {
            board[r][c] = Some(SpecialSquare::Escape);
        }
        board[5][5] = Some(SpecialSquare::Throne);

        let state: Vec<Vec<Option<Piece>>> = (0..NUM_TILES)
            .map(|r| {
                (0..NUM_TILES)
                    .map(|c| match initial_state[r][c] {
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

        (0..NUM_TILES).for_each(|r| {
            (0..NUM_TILES).for_each(|c| {
                let (x, y) = rowcol2xy(r, c);
                draw_rectangle(
                    x,
                    y,
                    TILE_SIZE,
                    TILE_SIZE,
                    if self.board[r][c].is_some() {
                        BROWN
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
                        draw_poly(
                            x + (TILE_SIZE + 1.) / 2.,
                            y + (TILE_SIZE + 1.) / 2.,
                            4,
                            TILE_SIZE / 2.3,
                            90.,
                            WHITE,
                        );
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
        Self::new([
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
        ])
    }
}
