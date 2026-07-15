use macroquad::prelude::*;
use std::collections::{HashSet, VecDeque};

use crate::board::Team::Attacker;
use crate::cons::{NUM_TILES, TILE_SIZE};
use crate::game::GameState;

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
    fn get_legal_moves(&self, row: usize, col: usize, board: &Board) -> HashSet<(usize, usize)> {
        let mut legal_moves: HashSet<(usize, usize)> = HashSet::new();
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
        legal_moves
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
    pub fn get_legal_moves(
        &self,
        row: usize,
        col: usize,
        board: &Board,
    ) -> HashSet<(usize, usize)> {
        match &self {
            Piece::Soldier(p) => p.get_legal_moves(row, col, board),
            Piece::King(p) => p.get_legal_moves(row, col, board),
        }
    }

    pub fn get_team(&self) -> Team {
        match &self {
            Piece::Soldier(p) => p.team,
            Piece::King(p) => p.team,
        }
    }
    fn is_hostile_square(&self, row: i32, col: i32, board: &Board) -> bool {
        // Check bounds
        if row < 0 || col < 0 || row >= NUM_TILES as i32 || col >= NUM_TILES as i32 {
            return false;
        }

        let row = row as usize;
        let col = col as usize;

        // Special Squares
        if let Some(s) = &board.board[row][col] {
            // If the piece is a defender and the throne is occupied, then it's not and
            // hostile square
            if matches!(s, SpecialSquare::Throne)
                && board.state[row][col].is_some()
                && self.get_team() == Team::Defender
            {
                return false;
            }

            return true;
        }

        // Enemy present
        board.state[row][col]
            .map(|p| p.get_team() != self.get_team())
            .unwrap_or(false)
    }

    pub fn is_captured(
        &self,
        row: i32,
        col: i32,
        board: &mut Board,
        origin_row: i32,
        origin_col: i32,
    ) -> GameState {
        match board.state[row as usize][col as usize] {
            None => GameState::Playing,
            Some(p) => {
                match p {
                    Piece::Soldier(_) => {
                        // Only check hostile squares on the opposite side of the origin_square
                        if self.is_hostile_square(origin_row, origin_col, board)
                            && self.is_hostile_square(
                                row + (row - origin_row),
                                col + (col - origin_col),
                                board,
                            )
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
                            return GameState::GameOver;
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
    pub fn new(
        initial_state: [[u8; NUM_TILES]; NUM_TILES],
        selected_square: Option<(usize, usize)>,
    ) -> Board {
        if let Some((row, col)) = selected_square {
            assert!(row < NUM_TILES && col < NUM_TILES)
        };

        let mut board: Vec<Vec<Option<SpecialSquare>>> = (0..NUM_TILES)
            .map(|_| -> Vec<Option<SpecialSquare>> { (0..NUM_TILES).map(|_| None).collect() })
            .collect();

        for (r, c) in [
            (0, 0),
            (0, NUM_TILES - 1),
            (NUM_TILES - 1, 0),
            (NUM_TILES - 1, NUM_TILES - 1),
        ] {
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
            selected_square,
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

    pub fn has_possible_moves(&self, team: &Team) -> bool {
        for r in 0..NUM_TILES {
            for c in 0..NUM_TILES {
                if let Some(p) = self.state[r][c]
                    && &p.get_team() == team
                    && !p.get_legal_moves(r, c, self).is_empty()
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_possible_moves(&self, team: &Team) -> Vec<Board> {
        let mut possible_moves = vec![];
        for r in 0..NUM_TILES {
            for c in 0..NUM_TILES {
                if let Some(p) = self.state[r][c]
                    && &p.get_team() == team
                {
                    for (move_r, move_c) in p.get_legal_moves(r, c, self) {
                        let (b, _, _) =
                            Self::get_board_after_move_piece(self, move_r, move_c, r, c, team);
                        possible_moves.push(b);
                    }
                }
            }
        }
        possible_moves
    }

    fn get_board_after_move_piece(
        board: &Board,
        row: usize,
        col: usize,
        player_row: usize,
        player_col: usize,
        player_turn: &Team,
    ) -> (Board, GameState, Team) {
        let mut new_board = Board::new(board.get_state_as_int(), None);

        new_board.state[row][col] = new_board.state[player_row][player_col];
        new_board.state[player_row][player_col] = None;

        // Check if the king is in a special square
        if let (Some(Piece::King(_)), Some(SpecialSquare::Escape)) =
            (&new_board.state[row][col], &new_board.board[row][col])
        {
            return (new_board, GameState::GameOver, Team::Defender);
        } else if board.is_surrounded() {
            return (new_board, GameState::GameOver, Team::Attacker);
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
                let Some(p) = new_board.state[new_row as usize][new_col as usize] else {
                    continue;
                };
                if p.is_captured(new_row, new_col, &mut new_board, row as i32, col as i32)
                    == GameState::GameOver
                {
                    return (new_board, GameState::GameOver, Team::Attacker);
                }
            }
        };

        (
            new_board,
            GameState::Playing,
            match player_turn {
                Team::Attacker => Team::Defender,
                Team::Defender => Team::Attacker,
            },
        )
    }

    pub fn is_surrounded(&self) -> bool {
        let mut defenders: HashSet<(usize, usize)> = HashSet::new();
        for r in 0..NUM_TILES {
            for c in 0..NUM_TILES {
                if let Some(p) = self.state[r][c]
                    && p.get_team() == Team::Defender
                {
                    defenders.insert((r, c));
                }
            }
        }
        assert!(!defenders.is_empty());

        // BFS first element in defenders.
        // If all other defenders were visited, then it's surrounded by a continuous
        // curve
        let mut queue: VecDeque<(usize, usize)> = VecDeque::from(vec![
            defenders
                .take(&defenders.iter().next().cloned().unwrap())
                .unwrap(),
        ]);

        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        while !queue.is_empty() {
            if let Some((r, c)) = queue.pop_front() {
                if visited.contains(&(r, c)) {
                    continue;
                }
                visited.insert((r, c));

                if let Some(p) = self.state[r][c]
                    && p.get_team() == Team::Attacker
                {
                    continue;
                }
                defenders.remove(&(r, c));

                // At least one piece can reach the edge, it's not surrounded
                if r == 0 || c == 0 || r == NUM_TILES - 1 || c == NUM_TILES - 1 {
                    return false;
                }

                // Add neighbours to the queue
                queue.push_back((r - 1, c));
                queue.push_back((r + 1, c));
                queue.push_back((r, c - 1));
                queue.push_back((r, c + 1));
            }
        }

        defenders.is_empty()
    }

    // TODO: This is very memory wasteful, implement Zobrist hashing instead
    pub fn get_state_as_int(&self) -> [[u8; NUM_TILES]; NUM_TILES] {
        std::array::from_fn(|irow| {
            std::array::from_fn(|icol| match &self.state[irow][icol] {
                Some(Piece::King(_)) => 3,
                Some(piece) if piece.get_team() == Attacker => 1,
                Some(_) => 2,
                None => 0,
            })
        })
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new(
            [
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
            ],
            None,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::board::*;
    #[test]
    fn test_has_squares_to_move_to() {
        assert!(
            !Board::new(
                [
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [2, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0],
                    [1, 2, 0, 0, 0, 0, 2, 1, 2, 0, 0],
                    [2, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ],
                None
            )
            .has_possible_moves(&Team::Attacker)
        );

        assert!(
            Board::new(
                [
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [2, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0],
                    [1, 2, 0, 0, 0, 0, 0, 1, 2, 0, 0],
                    [2, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ],
                None
            )
            .has_possible_moves(&Team::Attacker)
        );
    }

    #[test]
    fn test_surround() {
        // Single curve surrounded
        assert!(
            Board::new(
                [
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0],
                    [0, 1, 0, 2, 3, 0, 1, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0],
                    [0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ],
                None
            )
            .is_surrounded()
        );

        // 2 curves - not single curve surrounded
        assert!(
            !Board::new(
                [
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
                    [0, 1, 1, 1, 1, 1, 0, 1, 2, 0, 1],
                    [0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1],
                    [0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                    [0, 1, 0, 2, 3, 1, 0, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                    [0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ],
                None
            )
            .is_surrounded()
        );

        // 1 piece not surrounded
        assert!(
            !Board::new(
                [
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
                    [0, 1, 1, 1, 1, 1, 0, 0, 2, 0, 1],
                    [0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1],
                    [0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                    [0, 1, 0, 2, 3, 1, 0, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                    [0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ],
                None
            )
            .is_surrounded()
        );
    }

    #[test]
    fn test_get_state_as_int() {
        // Single curve surrounded
        assert!(
            Board::new(
                [
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0],
                    [0, 1, 0, 2, 3, 0, 1, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0],
                    [0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ],
                None
            )
            .get_state_as_int()
                == [
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0],
                    [0, 1, 0, 2, 3, 0, 1, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0],
                    [0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ],
        );
    }
}
