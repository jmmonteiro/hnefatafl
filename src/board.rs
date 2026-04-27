struct Square {
    soldier_can_stand: bool,
    escape_square: bool,
    throne: bool,
}

enum PieceType {
    Soldiar,
    King,
}
enum Team {
    Attacker,
    Defender,
}

struct Piece {
    piece_type: PieceType,
    team: Team,
}

struct Board {
    board: Vec<Vec<Square>>,
    state: Vec<Vec<Option<Piece>>>,
}

impl Board {
    pub fn new() -> Board {
        let mut board: Vec<Vec<Square>> = (0..11)
            .map(|_| -> Vec<Square> {
                (0..11)
                    .map(|_| Square {
                        soldier_can_stand: true,
                        escape_square: false,
                        throne: false,
                    })
                    .collect()
            })
            .collect();

        for (r, c) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
            board[r][c] = Square {
                soldier_can_stand: false,
                escape_square: true,
                throne: false,
            };
        }
        board[5][5] = Square {
            soldier_can_stand: false,
            escape_square: false,
            throne: true,
        };

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
                        1 => Some(Piece {
                            piece_type: PieceType::Soldiar,
                            team: Team::Attacker,
                        }),
                        2 => Some(Piece {
                            piece_type: PieceType::Soldiar,
                            team: Team::Defender,
                        }),
                        3 => Some(Piece {
                            piece_type: PieceType::King,
                            team: Team::Defender,
                        }),
                        _ => None,
                    })
                    .collect()
            })
            .collect();

        Board { board, state }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
