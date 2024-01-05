pub enum Side {
    KingSide,
    QueenSide,
}

#[derive(Copy, Clone)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn std_str(&self) -> String {
        format!(
            "{}{}",
            match self.x {
                0 => 'A',
                1 => 'B',
                2 => 'C',
                3 => 'D',
                4 => 'E',
                5 => 'F',
                6 => 'G',
                7 => 'H',
                _ => '?',
            },
            self.y + 1
        )
    }
}
