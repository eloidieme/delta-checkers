mod board;
mod capture;
mod moves;
mod position;
mod piece;
mod player;

use crate::board::Board;
use crate::position::Position;

fn main() {
    let board: Board = Board::new();
    let position: Position = Position::new(2, 3);
    let regular_moves = board.get_regular_moves(position);
    let capture_moves = board.get_capture_moves(position);

    println!("{:?}", regular_moves);
    println!("{:?}", capture_moves);
}
