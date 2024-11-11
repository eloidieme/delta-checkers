#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
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