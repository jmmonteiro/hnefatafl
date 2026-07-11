use std::collections::HashMap;

use crate::{
    board::{Board, Piece, SpecialSquare, Team},
    cons::NUM_TILES,
    game::GameState,
};
struct Tree {
    root: [u8; NUM_TILES * NUM_TILES],
}
impl Tree {
    fn new(root: [u8; NUM_TILES * NUM_TILES]) -> Tree {
        Tree { root }
    }
}

struct Node {
    N: u128,
    Q: f64,
    children: Vec<[u8; NUM_TILES * NUM_TILES]>,
    is_terminal: bool,
}

impl Node {
    fn expansion(&mut self, transposition_table: HashMap<[u8; NUM_TILES * NUM_TILES], Node>) {

        // get possible moves
        // if possible move is not in list of children
    }
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
        return (new_board, GameState::GameOver, *player_turn);
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
                return (new_board, GameState::GameOver, *player_turn);
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
