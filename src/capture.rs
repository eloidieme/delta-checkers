use crate::board::{Board, Move, Player, Piece, Position, BOARD_SIDE}; 

pub struct CaptureSearch<'a> {
    board: &'a Board,
    player: Player,
    is_king: bool,
}

impl<'a> CaptureSearch<'a> {
    pub fn new(board: &'a Board, player: Player, is_king: bool) -> Self {
        CaptureSearch {
            board,
            player,
            is_king,
        }
    }

    pub fn find_captures(
        &self,
        current_pos: Position,
        mut visited_positions: Vec<Position>,
        captured_pieces: Vec<Position>,
    ) -> Vec<Move> {
        let mut moves = Vec::new();
        visited_positions.push(current_pos);

        for diagonal in current_pos.get_diagonals(BOARD_SIDE) {
            let jumped_pos = Position::new(
                (current_pos.row + diagonal.row) / 2,
                (current_pos.col + diagonal.col) / 2
            );

            if let Some(jumped_piece) = self.board.get(jumped_pos) {
                if self.is_valid_capture(jumped_piece, current_pos, diagonal, &jumped_pos, &captured_pieces) {
                    let mut new_captured = captured_pieces.clone();
                    new_captured.push(jumped_pos);
                    
                    let next_captures = self.find_captures(
                        diagonal,
                        visited_positions.clone(),
                        new_captured.clone()
                    );

                    if next_captures.is_empty() {
                        // If no more captures possible, save this sequence
                        moves.push(Move::new(visited_positions[0], visited_positions.clone(), Some(new_captured)));
                    } else {
                        moves.extend(next_captures);
                    }
                }
            }
        }
        moves
    }

   fn is_valid_capture(
        &self,
        jumped_piece: &Piece,
        current_pos: Position,
        diagonal: Position,
        jumped_pos: &Position,
        captured_pieces: &[Position],
    ) -> bool {
        jumped_piece.player() != self.player 
        && self.board.get(diagonal).is_none()
        && (self.is_king || self.player.is_forward_move(current_pos, diagonal))
        && !captured_pieces.contains(jumped_pos)
    }
}