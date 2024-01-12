use super::{color::Color, piece::Piece};
use crate::utils::Pos;
use std::mem::discriminant;

pub struct Board {
    content: [[Option<Piece>; 8]; 8],
    pub trait_color: Color,
}

impl Board {
    pub fn new() -> Self {
        todo!()
    }

    pub fn get_square(&self, pos: Pos) -> Option<Piece> {
        self.content[pos.x][pos.y]
    }

    pub fn find_pieces(&self, piece: Piece) -> Vec<Pos> {
        let mut pos_vec = vec![];
        for x in 0..7 {
            for y in 0..7 {
                let pos = Pos::new(x, y);
                if let Some(current_piece) = self.get_square(pos) {
                    if discriminant(&current_piece) == discriminant(&piece) {
                        pos_vec.push(Pos::new(x, y));
                    }
                }
            }
        }
        return pos_vec;
    }
}
