use crate::position::Position;

#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn is_forward_move(&self, src: Position, dst: Position) -> bool {
        match self {
            Player::Black => dst.row > src.row,
            Player::White => dst.row < src.row,
        }
    }
}