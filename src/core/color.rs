use Color::*;

#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn inverse(&self) -> Color {
        match self {
            White => Black,
            Black => White,
        }
    }
}
