use crate::capture::CaptureSearch;
use crate::moves::Move;
use crate::piece::Piece;
use crate::position::Position;
use crate::player::Player;

pub const BOARD_SIDE: usize = 8;
pub const BOARD_SIZE: usize = BOARD_SIDE*BOARD_SIDE;

pub struct Board {
    grid: [Option<Piece>; BOARD_SIZE],
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
                .map(|diag| Move::new(src, vec![diag], None))
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

