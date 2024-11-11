use crate::capture::CaptureSearch;

pub const BOARD_SIDE: usize = 8;
pub const BOARD_SIZE: usize = BOARD_SIDE*BOARD_SIDE;

pub struct Board {
    grid: [Option<Piece>; BOARD_SIZE],
}

#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    Black,
    White,
}

#[derive(Clone, Copy)]
pub struct Piece {
    player: Player,
    is_king: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct Move {
    src: Position,
    stops: Vec<Position>,
    captures: Option<Vec<Position>>,
}

impl Move {
    pub fn new(src: Position, stops: Vec<Position>, captures: Option<Vec<Position>>) -> Move {
        Move {
            src,
            stops,
            captures,
        }
    }
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    pub fn from_index(idx: usize, board_side: usize) -> Self {
        Position {
            row: idx / board_side,
            col: idx % board_side
        }
    }

    pub fn to_index(&self, board_side: usize) -> usize {
        self.row * board_side + self.col
    }

    pub fn is_valid(&self, board_side: usize) -> bool {
        self.row < board_side && self.col < board_side
    }

    pub fn get_diagonals(&self, board_side: usize) -> Vec<Position> {
        if !self.is_valid(board_side) {
            panic!("Trying to get diagonals of invalid position");
        }

        const TOP_LEFT: (i32, i32) = (-1, -1);
        const TOP_RIGHT: (i32, i32) = (-1, 1);
        const BOTTOM_LEFT: (i32, i32) = (1, -1);
        const BOTTOM_RIGHT: (i32, i32) = (1, 1);

        const DIAGONAL_OFFSETS: [(i32, i32); 4] = [
            TOP_LEFT,
            TOP_RIGHT,
            BOTTOM_LEFT,
            BOTTOM_RIGHT,
        ];

        DIAGONAL_OFFSETS.iter()
            .filter_map(|&(row_offset, col_offset)| {
                let new_row = self.row as i32 + row_offset;
                let new_col = self.col as i32 + col_offset;

                if (0..board_side as i32).contains(&new_row) 
                   && (0..board_side as i32).contains(&new_col) {
                    Some(Position::new(new_row as usize, new_col as usize))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Piece {
    pub fn new(player: Player) -> Self {
        Piece {
            player,
            is_king: false,
        }
    }

    pub fn player(&self) -> Player {
        self.player
    }

    pub fn is_king(&self) -> bool {
        self.is_king
    }
}

impl Player {
    pub fn is_forward_move(&self, src: Position, dst: Position) -> bool {
        match self {
            Player::Black => dst.row > src.row,
            Player::White => dst.row < src.row,
        }
    }
}

impl Board {
    pub fn new() -> Self {
        let mut grid = [None; BOARD_SIZE];

        for idx in 0..BOARD_SIZE {
            let pos: Position = Position::from_index(idx, BOARD_SIDE);
            if (pos.row + pos.col) % 2 != 0 {
                match pos.row {
                    0..=2 => {grid[idx] = Some(Piece::new(Player::Black))}
                    5..=7 => {grid[idx] = Some(Piece::new(Player::White))}
                    _ => ()
                }
            }
        }

        Board { grid }
    }

    pub fn get(&self, pos: Position) -> Option<&Piece> {
        if !pos.is_valid(BOARD_SIDE) {
            panic!("Attempted to access invalid position: row={}, col={}", pos.row, pos.col);
        }
        self.grid[pos.to_index(BOARD_SIDE)].as_ref()
    }

    pub fn set(&mut self, pos: Position, piece: Option<Piece>) {
        if !pos.is_valid(BOARD_SIDE) {
            panic!("Attempted to set invalid position: row={}, col={}", pos.row, pos.col);
        }
        self.grid[pos.to_index(BOARD_SIDE)] = piece;
    }    

    pub fn get_regular_moves(&self, src: Position) -> Option<Vec<Move>> {
        let piece = self.get(src)?;
        
        let only_forward = !piece.is_king();
        let player = piece.player();

        Some(
            src.get_diagonals(BOARD_SIDE)
                .into_iter()
                .filter(|diag| {
                    self.get(*diag).is_none() && 
                    (!only_forward || player.is_forward_move(src, *diag))
                })
                .map(|diag| Move {
                    src,
                    stops: vec![diag],
                    captures: None,
                })
                .collect()
        )
    }

    pub fn get_capture_moves(&self, src: Position) -> Option<Vec<Move>> {
        let piece = self.get(src)?;
        
        let searcher = CaptureSearch::new(
            self,
            piece.player(),
            piece.is_king()
        );
 
        let captures = searcher.find_captures(
            src,
            Vec::new(),
            Vec::new()
        );
 
        if captures.is_empty() {
            None
        } else {
            Some(captures)
        }
    }
}

