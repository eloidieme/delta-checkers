use crate::position::Position;

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