use crate::player::Player;

#[derive(Clone, Copy)]
pub struct Piece {
    player: Player,
    is_king: bool,
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