use super::color::Color;
use super::piece::Piece::*;
use std::mem::discriminant;

pub struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

pub struct Board {
    content: [[Option<Piece>; 8]; 8],
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
