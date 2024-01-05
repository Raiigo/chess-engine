use super::color::Color;
use Piece::*;

#[derive(Copy, Clone)]
pub enum Piece {
    King { color: Color, castled: bool },
    Queen { color: Color },
    Bishop { color: Color },
    Knight { color: Color },
    Rook { color: Color, castled: bool },
    Pawn { color: Color, en_passant: bool },
}

impl Piece {
    pub fn color(&self) -> Color {
        match self {
            King { color, castled: _ } => *color,
            Queen { color } => *color,
            Bishop { color } => *color,
            Knight { color } => *color,
            Rook { color, castled: _ } => *color,
            Pawn {
                color,
                en_passant: _,
            } => *color,
        }
    }
}
