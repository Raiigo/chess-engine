use super::piece::Piece;
use crate::utils::{Pos, Side};

pub enum SpecialMove {
    Promote(Piece),
    Castle(Side),
    EnPassant,
}

pub struct Move {
    pub start_pos: Pos,
    pub end_pos: Pos,
    pub special: Option<SpecialMove>,
}

impl Move {
    pub fn new(start_pos: Pos, end_pos: Pos) -> Option<Self> {
        let new_move = Self {
            start_pos,
            end_pos,
            special: None,
        };
        if new_move.is_valid() {
            Some(new_move)
        } else {
            None
        }
    }

    pub fn is_valid(&self) -> bool {
        todo!()
    }
}
