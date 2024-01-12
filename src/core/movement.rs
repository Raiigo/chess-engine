use super::piece::Piece;
use super::piece::Piece::*;
use super::{board::Board, color::Color};
use crate::utils::{Pos, Side};
use SpecialMove::*;

pub enum SpecialMove {
    Promote(Piece),
    Castle(Side),
    EnPassant,
}

pub struct Move<'a> {
    /// start_pos is always the position of a piece (in the case of a castle, start_pos is the position of the King)
    pub start_pos: Pos,
    pub end_pos: Pos,
    pub special_move: Option<SpecialMove>,
    pub board_ref: &'a Board,
}

impl<'a> Move<'a> {
    pub fn new(start_pos: Pos, end_pos: Pos, board_ref: &'a Board) -> Option<Self> {
        let new_move = Self {
            start_pos,
            end_pos,
            special_move: None,
            board_ref,
        };
        if new_move.is_valid() {
            Some(new_move)
        } else {
            None
        }
    }

    pub fn is_valid(&self) -> bool {
        // Check trait color
        if self.board_ref.trait_color != self.moving_piece().color() {
            return false;
        }
    }

    pub fn moving_piece(&self) -> Piece {
        if let Some(piece) = self.board_ref.get_square(self.start_pos) {
            piece
        } else {
            panic!("A piece should have been found, there is an error in the verification process of the move");
        }
    }

    pub fn gain(&self) -> Option<Piece> {
        if let Some(EnPassant) = self.special_move {
            Some(Pawn {
                color: self.moving_piece().color().inverse(),
                en_passant: true,
            })
        } else {
            self.board_ref.get_square(self.end_pos)
        }
    }
}
